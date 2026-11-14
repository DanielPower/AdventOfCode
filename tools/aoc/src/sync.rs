//! Keeps the private data repository pulled, committed and pushed, so it never
//! needs managing by hand:
//!
//! * anything that writes data (`new`, `fetch`, `data import`) pulls first and
//!   commits + pushes afterwards;
//! * `test` and `run` pull (at most every few minutes) when data is missing;
//! * the solutions repo's pre-push hook records answers for the solutions being
//!   pushed and pushes the data repo first, so CI can verify them.
//!
//! Sync problems are reported but never fail the command: changes stay
//! committed locally and the next sync pushes them. Disabled in CI and with
//! `AOC_DATA_SYNC=off`.

use crate::ctx::Ctx;
use crate::data;
use crate::manifest::Impl;
use crate::model::Solution;
use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};

/// How often `test`/`run` may pull when they find data missing.
const REFRESH_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Variables git sets for hooks; inherited, they would point git commands in
/// the data repo at the solutions repo instead.
const GIT_REPO_VARS: &[&str] = &[
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_PREFIX",
    "GIT_COMMON_DIR",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_NAMESPACE",
    "GIT_QUARANTINE_PATH",
];

fn git(dir: &Path, args: &[&str]) -> Result<Output> {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(dir).args(args);
    for var in GIT_REPO_VARS {
        cmd.env_remove(var);
    }
    let out = cmd.output().context("running git")?;
    if !out.status.success() {
        // git's first line of complaint is the useful one ("fatal: ...").
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
        let first = text
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .unwrap_or("failed");
        bail!("git {}: {first}", args[0]);
    }
    Ok(out)
}

fn git_text(dir: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8_lossy(&git(dir, args)?.stdout)
        .trim()
        .to_string())
}

pub fn enabled(ctx: &Ctx) -> bool {
    let ci =
        std::env::var("CI").is_ok_and(|v| v == "true") || std::env::var("GITHUB_ACTIONS").is_ok();
    !ci && ctx
        .var("AOC_DATA_SYNC")
        .is_none_or(|v| v != "off" && v != "0")
}

fn is_repo(dir: &Path) -> bool {
    dir.join(".git").exists()
}

static WARNED: AtomicBool = AtomicBool::new(false);

/// Sync is on and the data directory is a git repository.
fn ready(ctx: &Ctx) -> bool {
    if !enabled(ctx) {
        return false;
    }
    if !is_repo(&ctx.data_dir) {
        if ctx.data_dir.exists() && !WARNED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "data: {} isn't a git repository, so it isn't synced (see `cargo aoc setup`)",
                ctx.data_dir.display()
            );
        }
        return false;
    }
    true
}

fn upstream(dir: &Path) -> Option<String> {
    git_text(
        dir,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    )
    .ok()
}

/// Bring the data repository up to date. Returns false if that failed.
pub fn pull(ctx: &Ctx) -> bool {
    if !ready(ctx) {
        return true;
    }
    let dir = &ctx.data_dir;
    if upstream(dir).is_none() {
        return true; // nothing pushed yet (e.g. a fresh, empty remote)
    }
    // Commit local changes first, so the pull can only rebase commits. (Never
    // stash: a conflicting stash pop leaves conflict markers that would then
    // be committed.)
    if let Err(e) = commit(dir) {
        eprintln!("data: {e:#}");
        return false;
    }
    rebase_onto_remote(dir)
}

/// `git pull --rebase`; on conflict, abort and keep the local commits.
fn rebase_onto_remote(dir: &Path) -> bool {
    match git(dir, &["pull", "--quiet", "--rebase"]) {
        Ok(_) => true,
        Err(e) => {
            let conflicts =
                git_text(dir, &["diff", "--name-only", "--diff-filter=U"]).unwrap_or_default();
            let _ = git(dir, &["rebase", "--abort"]);
            if conflicts.is_empty() {
                eprintln!("data: couldn't update from the remote ({e:#}); using the local copy");
            } else {
                eprintln!(
                    "data: local and remote both changed {}; nothing was overwritten. Resolve with\n  git -C {} pull --rebase",
                    conflicts.lines().collect::<Vec<_>>().join(", "),
                    dir.display()
                );
            }
            false
        }
    }
}

/// Pull, but at most every few minutes; for `test`/`run` when data is missing.
pub fn refresh(ctx: &Ctx) {
    if !ready(ctx) {
        return;
    }
    let stamp = ctx.cache_dir.join("data-pulled");
    let fresh = std::fs::metadata(&stamp)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_some_and(|age| age < REFRESH_INTERVAL);
    if fresh {
        return;
    }
    pull(ctx);
    let _ = std::fs::create_dir_all(&ctx.cache_dir);
    let _ = std::fs::write(&stamp, b"");
}

/// Commit everything in the data repository and push it.
pub fn save(ctx: &Ctx) {
    if !ready(ctx) {
        return;
    }
    if let Err(e) = commit_and_push(&ctx.data_dir) {
        eprintln!("data: {e:#}\ndata: changes are kept locally; the next sync retries");
    }
}

fn commit(dir: &Path) -> Result<()> {
    git(dir, &["add", "--all"])?;
    let staged = git_text(dir, &["diff", "--cached", "--name-only"])?;
    if !staged.is_empty() {
        let message = commit_message(&staged);
        git(dir, &["commit", "--quiet", "-m", &message])?;
        eprintln!("data: committed \"{message}\"");
    }
    Ok(())
}

fn commit_and_push(dir: &Path) -> Result<()> {
    commit(dir)?;
    if git_text(dir, &["remote"])?.is_empty() {
        bail!("the data repository has no remote to push to (see `cargo aoc setup`)");
    }
    if git(dir, &["rev-parse", "--verify", "--quiet", "HEAD"]).is_err() {
        return Ok(()); // no commits at all yet
    }
    match upstream(dir) {
        Some(_) => {
            if git_text(dir, &["rev-list", "--count", "@{u}..HEAD"])? != "0" {
                if git(dir, &["push", "--quiet"]).is_err() {
                    // Usually the remote moved on meanwhile: rebase, try once more.
                    if !rebase_onto_remote(dir) {
                        bail!("couldn't push");
                    }
                    git(dir, &["push", "--quiet"])?;
                }
                eprintln!("data: pushed");
            }
        }
        None => {
            git(
                dir,
                &["push", "--quiet", "--set-upstream", "origin", "HEAD"],
            )?;
            eprintln!("data: pushed");
        }
    }
    Ok(())
}

/// "2025/08, 2025/09" from paths like "2025/08/input.txt".
fn commit_message(staged: &str) -> String {
    let days: BTreeSet<String> = staged
        .lines()
        .map(|p| p.split('/').take(2).collect::<Vec<_>>().join("/"))
        .collect();
    let mut days: Vec<String> = days.into_iter().collect();
    if days.len() > 5 {
        let more = days.len() - 4;
        days.truncate(4);
        days.push(format!("{more} more"));
    }
    format!("Update {}", days.join(", "))
}

/// `cargo aoc data sync`: pull, then commit and push anything outstanding.
pub fn sync_now(ctx: &Ctx) -> Result<()> {
    if !enabled(ctx) {
        bail!("data sync is disabled (CI, or AOC_DATA_SYNC=off)");
    }
    if !is_repo(&ctx.data_dir) {
        bail!(
            "{} isn't a git repository; run `cargo aoc setup --data-repo <url>`",
            ctx.data_dir.display()
        );
    }
    pull(ctx);
    save(ctx);
    let status = git_text(&ctx.data_dir, &["status", "--short", "--branch"])?;
    eprintln!("data: {}", status.lines().next().unwrap_or(""));
    Ok(())
}

/// One line describing the data repository's state, for `doctor`.
pub fn describe(ctx: &Ctx) -> (bool, String) {
    let dir = &ctx.data_dir;
    if !is_repo(dir) {
        return (
            false,
            format!(
                "{} isn't a git repository (cargo aoc setup --data-repo <url>)",
                dir.display()
            ),
        );
    }
    let remote = git_text(dir, &["remote", "get-url", "origin"]).unwrap_or_default();
    if remote.is_empty() {
        return (false, "the data repository has no `origin` remote".into());
    }
    let dirty = !git_text(dir, &["status", "--porcelain"])
        .unwrap_or_default()
        .is_empty();
    let ahead =
        git_text(dir, &["rev-list", "--count", "@{u}..HEAD"]).unwrap_or_else(|_| "?".into());
    match (dirty, ahead.as_str()) {
        (false, "0") => (true, format!("data repository in sync with {remote}")),
        _ => (
            false,
            format!(
                "data repository has unsynced changes (uncommitted: {dirty}, unpushed commits: {ahead}); run `cargo aoc data sync`"
            ),
        ),
    }
}

/// `cargo aoc setup`: clone the data repository and install the git hooks.
pub fn setup(ctx: &Ctx, data_repo: Option<&str>) -> Result<()> {
    let dir = &ctx.data_dir;
    match (is_repo(dir), data_repo) {
        (true, Some(url)) => {
            if git_text(dir, &["remote"])?.is_empty() {
                git(dir, &["remote", "add", "origin", url])?;
                eprintln!("data: added remote {url}");
            } else {
                eprintln!(
                    "data: {} is already a clone; leaving it as is",
                    dir.display()
                );
            }
        }
        (true, None) => eprintln!("data: using the existing clone at {}", dir.display()),
        (false, Some(url)) => {
            if dir.exists() && std::fs::read_dir(dir)?.next().is_some() {
                bail!(
                    "{} exists and isn't empty; move it aside or set AOC_DATA_DIR",
                    dir.display()
                );
            }
            let parent = dir.parent().context("bad data directory")?;
            std::fs::create_dir_all(parent)?;
            git(parent, &["clone", "--quiet", url, &dir.to_string_lossy()])?;
            eprintln!("data: cloned {url} into {}", dir.display());
        }
        (false, None) => bail!(
            "no data repository at {}: create an empty private repository on GitHub, then run\n  cargo aoc setup --data-repo git@github.com:<you>/AdventOfCode-data.git",
            dir.display()
        ),
    }

    git(&ctx.root, &["config", "core.hooksPath", ".githooks"])?;
    eprintln!("hooks: installed (pre-push records answers and pushes the data repository)");

    // Backfill: download data for every solution that lacks it (one-time;
    // re-running setup later only fetches what's still missing).
    let impls = crate::manifest::load_all(&ctx.root)?;
    let missing = missing_days(ctx, &impls, |_, _| true)?;
    match (missing.is_empty(), ctx.var("AOC_SESSION").is_some()) {
        (true, _) => {
            pull(ctx);
            save(ctx);
        }
        (false, true) => {
            eprintln!(
                "data: downloading inputs/answers for {} day(s) with solutions ({}); about {}s",
                missing.len(),
                day_list(&missing),
                missing.len() * 6
            );
            let days: Vec<(u16, u8)> = missing.into_iter().collect();
            data::fetch_days(ctx, &days, false)?;
        }
        (false, false) => eprintln!(
            "note: {} day(s) with solutions have no input/answers yet. Put your adventofcode.com \
             session cookie in .env (AOC_SESSION=...) and run `cargo aoc setup` again to download them",
            missing.len()
        ),
    }
    let old_layout = std::fs::read_dir(&ctx.root)?
        .filter_map(|e| e.ok())
        .any(|e| e.path().join("data").is_dir() || e.path().join("inputs").is_dir());
    if old_layout {
        eprintln!(
            "note: found old-layout input files; `cargo aoc data import` moves them into the data repository"
        );
    }
    Ok(())
}

pub fn hooks_installed(ctx: &Ctx) -> bool {
    git_text(&ctx.root, &["config", "--get", "core.hooksPath"]).is_ok_and(|p| p == ".githooks")
}

/// The pre-push hook: for the solutions being pushed, download any missing
/// inputs and answers, then push the data repository so CI can verify them.
/// Reads git's "<local ref> <local sha> <remote ref> <remote sha>" lines from stdin.
pub fn pre_push(ctx: &Ctx, impls: &[Impl], stdin: &str) -> Result<()> {
    if !enabled(ctx) {
        return Ok(());
    }
    let changed = pushed_paths(ctx, stdin)?;
    let missing = missing_days(ctx, impls, |imp, sol| {
        let manifest = format!("{}/aoc.toml", ctx.rel(&imp.dir).to_string_lossy());
        changed.contains(&manifest) || touches(ctx, imp, sol, &changed)
    })?;
    let can_fetch = ctx.var("AOC_SESSION").is_some();
    if missing.is_empty() || !can_fetch || missing.len() > HOOK_FETCH_LIMIT {
        if !missing.is_empty() {
            let how = if can_fetch {
                "too many to download during a push; run `cargo aoc setup` to download them all"
            } else {
                "set AOC_SESSION (in .env) so they're downloaded automatically"
            };
            eprintln!(
                "data: no input or answer recorded for {}; {how}",
                day_list(&missing)
            );
        }
        pull(ctx);
        save(ctx);
        return Ok(());
    }
    eprintln!(
        "data: recording inputs/answers for {} before pushing",
        day_list(&missing)
    );
    let days: Vec<(u16, u8)> = missing.into_iter().collect();
    data::fetch_days(ctx, &days, false)
}

/// At most this many days are downloaded by the pre-push hook (2 requests each).
const HOOK_FETCH_LIMIT: usize = 6;

/// Unlocked days with a solution (matching `filter`) but no input, or no
/// answer for that solution's part.
fn missing_days(
    ctx: &Ctx,
    impls: &[Impl],
    filter: impl Fn(&Impl, Solution) -> bool,
) -> Result<BTreeSet<(u16, u8)>> {
    let mut missing = BTreeSet::new();
    for imp in impls {
        for &sol in &imp.solutions {
            if !data::unlocked(imp.year, sol.day) || !filter(imp, sol) {
                continue;
            }
            let has_input = data::input(ctx, imp.year, sol.day)?.is_some();
            let has_answer = data::answers(ctx, imp.year, sol.day)?
                .expected(sol.part)
                .is_some();
            if !has_input || !has_answer {
                missing.insert((imp.year, sol.day));
            }
        }
    }
    Ok(missing)
}

/// "2025/01, 2025/02, ... and 7 more"
fn day_list(days: &BTreeSet<(u16, u8)>) -> String {
    let shown: Vec<String> = days
        .iter()
        .take(5)
        .map(|(y, d)| format!("{y}/{d:02}"))
        .collect();
    match days.len().saturating_sub(5) {
        0 => shown.join(", "),
        more => format!("{} and {more} more", shown.join(", ")),
    }
}

fn touches(ctx: &Ctx, imp: &Impl, sol: Solution, changed: &BTreeSet<String>) -> bool {
    let src = ctx
        .rel(&imp.sources_path(sol))
        .to_string_lossy()
        .replace('\\', "/");
    let prefix = format!("{src}/");
    changed.iter().any(|p| *p == src || p.starts_with(&prefix))
}

/// Paths changed by the commits being pushed (in the solutions repository).
fn pushed_paths(ctx: &Ctx, stdin: &str) -> Result<BTreeSet<String>> {
    let solutions_git = |args: &[&str]| -> Result<String> {
        let out = Command::new("git")
            .current_dir(&ctx.root)
            .args(args)
            .output()?;
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    };
    let zero = |sha: &str| sha.chars().all(|c| c == '0');
    let mut paths = BTreeSet::new();
    for line in stdin.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [_, local, _, remote] = fields[..] else {
            continue;
        };
        if zero(local) {
            continue; // deleting a branch
        }
        let base = if zero(remote) {
            // A new branch: compare with the remote's default branch, if known.
            let base = solutions_git(&["merge-base", local, "refs/remotes/origin/HEAD"])?;
            let base = if base.trim().is_empty() {
                solutions_git(&["merge-base", local, "refs/remotes/origin/main"])?
            } else {
                base
            };
            base.trim().to_string()
        } else {
            remote.to_string()
        };
        let listing = if base.is_empty() {
            solutions_git(&["ls-tree", "-r", "--name-only", local])?
        } else {
            solutions_git(&["diff", "--name-only", &base, local])?
        };
        paths.extend(listing.lines().map(String::from));
    }
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages() {
        assert_eq!(
            commit_message("2025/08/input.txt\n2025/08/answers.toml"),
            "Update 2025/08"
        );
        assert_eq!(
            commit_message("2025/08/input.txt\n2024/16/input.txt"),
            "Update 2024/16, 2025/08"
        );
        let many = (1..=7)
            .map(|d| format!("2025/{d:02}/input.txt"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            commit_message(&many),
            "Update 2025/01, 2025/02, 2025/03, 2025/04, 3 more"
        );
    }
}
