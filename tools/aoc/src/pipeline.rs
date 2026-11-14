//! Selecting solutions, building them, and running them against the data.

use crate::ctx::Ctx;
use crate::data::{self, Answers, Expected};
use crate::engine::{Engine, Exec, Exit, Outcome, Phase};
use crate::manifest::{BUILD_TIMEOUT, Impl};
use crate::model::{Selector, Solution};
use crate::pool::parallel_map;
use crate::results::{self, Entry, Keyer, Status, Store};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub struct SelectOpts {
    pub sel: Selector,
    pub imp: Option<String>,
    pub since: Option<String>,
    /// Keep implementations with no matching solutions (for `list`).
    pub include_empty: bool,
}

/// (index into impls, selected solutions)
pub type Work = Vec<(usize, Vec<Solution>)>;

pub fn select(ctx: &Ctx, impls: &[Impl], o: &SelectOpts) -> Result<Work> {
    let changed = o
        .since
        .as_deref()
        .map(|r| changed_paths(ctx, r))
        .transpose()?;
    let mut work = Vec::new();
    for (i, imp) in impls.iter().enumerate() {
        if !o.sel.matches_year(imp.year) || o.imp.as_ref().is_some_and(|f| *f != imp.id) {
            continue;
        }
        if let Some(changed) = &changed {
            let prefix = format!("{}/", ctx.rel(&imp.dir).to_string_lossy());
            if !changed.iter().any(|p| p.starts_with(&prefix)) {
                continue;
            }
        }
        let sols: Vec<Solution> = imp
            .solutions
            .iter()
            .copied()
            .filter(|&s| o.sel.matches(imp.year, s))
            .collect();
        if !sols.is_empty() || o.include_empty {
            work.push((i, sols));
        }
    }
    Ok(work)
}

/// Files changed relative to the merge base with `git_ref`, including
/// uncommitted and untracked files.
fn changed_paths(ctx: &Ctx, git_ref: &str) -> Result<Vec<String>> {
    let git = |args: &[&str]| -> Result<String> {
        let out = Command::new("git")
            .current_dir(&ctx.root)
            .args(args)
            .output()
            .context("running git")?;
        if !out.status.success() {
            bail!(
                "git {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    };
    let base = git(&["merge-base", git_ref, "HEAD"])?;
    let mut paths: Vec<String> = git(&["diff", "--name-only", base.trim()])?
        .lines()
        .map(String::from)
        .collect();
    paths.extend(
        git(&["ls-files", "--others", "--exclude-standard"])?
            .lines()
            .map(String::from),
    );
    Ok(paths)
}

fn fmt_duration(d: Duration) -> String {
    if d < Duration::from_secs(1) {
        format!("{:.1}ms", d.as_secs_f64() * 1000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

fn indent(s: &str) -> String {
    s.lines()
        .map(|l| format!("      {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Build failures, keyed by (impl index, solution).
pub struct Built {
    pub images: HashMap<usize, String>,
    pub failed: HashMap<(usize, Solution), String>,
}

/// Prepare toolchain images and run build commands (deduplicated, in parallel).
pub fn build(engine: &Engine, impls: &[Impl], work: &Work, jobs: usize) -> Built {
    let mut built = Built {
        images: HashMap::new(),
        failed: HashMap::new(),
    };
    for (i, sols) in work {
        match engine.prepare_image(&impls[*i]) {
            Ok(image) => {
                built.images.insert(*i, image);
            }
            Err(e) => {
                eprintln!(
                    "❌ {}: toolchain unavailable\n{}",
                    impls[*i].name(),
                    indent(&format!("{e:#}"))
                );
                for &s in sols {
                    built.failed.insert((*i, s), "toolchain unavailable".into());
                }
            }
        }
    }

    struct Unit {
        i: usize,
        sol: Option<Solution>,
        sols: Vec<Solution>,
    }
    let mut units = Vec::new();
    for (i, sols) in work {
        let imp = &impls[*i];
        let Some(template) = &imp.manifest.build else {
            continue;
        };
        if !built.images.contains_key(i) {
            continue;
        }
        if imp.build_is_per_solution() {
            // Solutions whose build command expands identically share one build.
            let mut groups: BTreeMap<String, Vec<Solution>> = BTreeMap::new();
            for &s in sols {
                groups
                    .entry(imp.expand(template, Some(s), "", ""))
                    .or_default()
                    .push(s);
            }
            units.extend(groups.into_values().map(|sols| Unit {
                i: *i,
                sol: Some(sols[0]),
                sols,
            }));
        } else {
            units.push(Unit {
                i: *i,
                sol: None,
                sols: sols.clone(),
            });
        }
    }

    let outcomes = parallel_map(&units, jobs, |u| {
        let imp = &impls[u.i];
        let label = match u.sol {
            Some(s) if u.sols.len() == 1 => format!("{} {s}", imp.name()),
            Some(s) => format!("{} day {:02}", imp.name(), s.day),
            None => imp.name(),
        };
        let result = engine.exec(&Exec {
            imp,
            image: &built.images[&u.i],
            sol: u.sol,
            template: imp.manifest.build.as_deref().unwrap(),
            phase: Phase::Build,
            stdin: None,
            timeout: BUILD_TIMEOUT,
        });
        match &result {
            Ok(o) if o.ok() => eprintln!("   built {label} ({})", fmt_duration(o.elapsed)),
            // Compiler output contains source code, never puzzle input, so it is safe to show in CI.
            Ok(o) => eprintln!(
                "❌ build failed: {label} ({})\n{}",
                o.describe(),
                indent(&o.stderr_tail(40))
            ),
            Err(e) => eprintln!("❌ build failed: {label}\n{}", indent(&format!("{e:#}"))),
        }
        result.map(|o| o.ok()).unwrap_or(false)
    });
    for (u, ok) in units.iter().zip(outcomes) {
        if !ok {
            for &s in &u.sols {
                built.failed.insert((u.i, s), "build failed".into());
            }
        }
    }
    built
}

pub struct TestOpts {
    pub jobs: usize,
    /// Redact everything derived from inputs/answers; mask answers in GitHub logs.
    pub ci: bool,
    /// Skip solutions whose cache key matches results.json.
    pub changed: bool,
    pub examples: bool,
    pub save: bool,
    pub json: Option<PathBuf>,
}

/// A day's input (if downloaded) and recorded answers.
type DayData = (Option<std::sync::Arc<Vec<u8>>>, Answers);

struct Case {
    i: usize,
    sol: Solution,
    label: String,
    input: std::sync::Arc<Vec<u8>>,
    expected: Option<Expected>,
    key: Option<String>,
}

#[derive(Serialize)]
struct Record {
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    example: String,
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    ms: Option<u64>,
    #[serde(skip)]
    key: Option<String>,
}

pub fn test(ctx: &Ctx, engine: &Engine, impls: &[Impl], work: Work, o: &TestOpts) -> Result<bool> {
    let mut store = Store::load(&ctx.root)?;
    let mut keyer = Keyer::default();
    let mut records = Vec::new();
    let mut cases = Vec::new();
    let mut unchanged = 0;
    let mut day_data: HashMap<(u16, u8), DayData> = HashMap::new();

    for (i, sols) in &work {
        let imp = &impls[*i];
        for &sol in sols {
            let id = results::id(imp, sol);
            let no_data = |why: &str| {
                println!("❔ {} {sol}  {why}", imp.name());
                Record {
                    id: id.clone(),
                    example: String::new(),
                    status: Status::NoData,
                    ms: None,
                    key: None,
                }
            };
            if o.examples {
                let examples = data::examples(ctx, imp.year, sol.day)?;
                let mut any = false;
                for ex in examples {
                    if let Some(expected) = ex.answers.expected(sol.part) {
                        any = true;
                        cases.push(Case {
                            i: *i,
                            sol,
                            label: ex.name.clone(),
                            input: std::sync::Arc::new(ex.input),
                            expected: Some(expected),
                            key: None,
                        });
                    }
                }
                if !any {
                    records.push(no_data("no examples"));
                }
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(e) =
                day_data.entry((imp.year, sol.day))
            {
                let input = data::input(ctx, imp.year, sol.day)?.map(std::sync::Arc::new);
                let answers = data::answers(ctx, imp.year, sol.day)?;
                e.insert((input, answers));
            }
            let (input, answers) = &day_data[&(imp.year, sol.day)];
            let Some(input) = input else {
                records.push(no_data(&format!(
                    "no input (run `cargo aoc fetch {} {}`)",
                    imp.year, sol.day
                )));
                continue;
            };
            let expected = answers.expected(sol.part);
            let key = expected
                .as_ref()
                .map(|e| keyer.key(imp, sol, input, &e.as_key()));
            if o.changed
                && let (Some(k), Some(entry)) = (&key, store.get(imp, sol))
                && entry.key == *k
                && entry.status != Status::NoData
            {
                unchanged += 1;
                continue;
            }
            cases.push(Case {
                i: *i,
                sol,
                label: String::new(),
                input: input.clone(),
                expected,
                key,
            });
        }
    }

    if o.ci && std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true") {
        // Defense in depth: GitHub replaces these strings with *** in logs.
        for c in &cases {
            for s in c.expected.iter().flat_map(Expected::secrets) {
                println!("::add-mask::{s}");
            }
        }
    }
    if unchanged > 0 {
        println!(
            "   skipping {unchanged} unchanged solution(s) already in {}",
            results::FILE
        );
    }

    let mut build_work: BTreeMap<usize, Vec<Solution>> = BTreeMap::new();
    for c in &cases {
        let sols = build_work.entry(c.i).or_default();
        if !sols.contains(&c.sol) {
            sols.push(c.sol);
        }
    }
    let built = build(engine, impls, &build_work.into_iter().collect(), o.jobs);

    let outcomes = parallel_map(&cases, o.jobs, |c| {
        let imp = &impls[c.i];
        let name = match c.label.as_str() {
            "" => format!("{} {}", imp.name(), c.sol),
            ex => format!("{} {} [{ex}]", imp.name(), c.sol),
        };
        if built.failed.contains_key(&(c.i, c.sol)) {
            println!("❌ {name}  build error");
            return (Status::BuildError, None);
        }
        let timeout = imp
            .run_timeout(c.sol.day)
            .unwrap_or(Duration::from_secs(60));
        let run = engine.exec(&Exec {
            imp,
            image: &built.images[&c.i],
            sol: Some(c.sol),
            template: &imp.manifest.run,
            phase: Phase::Run,
            stdin: Some(&c.input),
            timeout,
        });
        let out = match run {
            Ok(out) => out,
            Err(e) => {
                println!("❌ {name}  could not run\n{}", indent(&format!("{e:#}")));
                return (Status::RuntimeError, None);
            }
        };
        let ms = Some(out.elapsed.as_millis() as u64);
        let status = classify(&out, c.expected.as_ref());
        report(&name, status, &out, c.expected.as_ref(), timeout, o.ci);
        (status, ms)
    });

    for (c, (status, ms)) in cases.iter().zip(outcomes) {
        records.push(Record {
            id: results::id(&impls[c.i], c.sol),
            example: c.label.clone(),
            status,
            ms,
            key: c.key.clone(),
        });
    }

    let count = |f: &dyn Fn(Status) -> bool| records.iter().filter(|r| f(r.status)).count();
    let failures = count(&|s| s.is_failure());
    println!(
        "\n{} passed, {failures} failed, {} without data{}",
        count(&|s| s == Status::Pass),
        count(&|s| s == Status::NoData),
        if unchanged > 0 {
            format!(", {unchanged} unchanged")
        } else {
            String::new()
        }
    );

    if o.save && !o.examples && !engine.native {
        for r in &records {
            if let (Some(key), false) = (&r.key, r.status == Status::NoData) {
                store.results.insert(
                    r.id.clone(),
                    Entry {
                        status: r.status,
                        ms: r.ms,
                        key: key.clone(),
                    },
                );
            }
        }
        store.prune(impls);
        store.save(&ctx.root)?;
    }
    if let Some(path) = &o.json {
        std::fs::write(path, serde_json::to_string_pretty(&records)? + "\n")?;
    }
    Ok(failures == 0)
}

fn classify(out: &Outcome, expected: Option<&Expected>) -> Status {
    match (&out.exit, expected) {
        (Exit::TimedOut, _) => Status::Timeout,
        (Exit::Code(0), Some(e)) if e.matches(&out.stdout) => Status::Pass,
        (Exit::Code(0), Some(_)) => Status::Fail,
        (Exit::Code(0), None) => Status::NoData,
        _ => Status::RuntimeError,
    }
}

/// Print one result line. In CI mode nothing derived from the input or answer
/// (expected value, stdout, stderr) is ever printed.
fn report(
    name: &str,
    status: Status,
    out: &Outcome,
    expected: Option<&Expected>,
    timeout: Duration,
    ci: bool,
) {
    let time = fmt_duration(out.elapsed);
    let mut msg = match status {
        Status::Pass => format!("{} {name}  {time}", status.icon()),
        Status::Timeout => format!("❌ {name}  timeout after {}", fmt_duration(timeout)),
        Status::RuntimeError => format!("❌ {name}  runtime error ({})", out.describe()),
        Status::Fail => format!("❌ {name}  wrong answer ({time})"),
        _ => format!("❔ {name}  no recorded answer ({time})"),
    };
    if !ci {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let shown: String = stdout
            .trim()
            .lines()
            .take(12)
            .collect::<Vec<_>>()
            .join("\n");
        match status {
            Status::Fail => {
                let exp = match expected {
                    Some(Expected::Answer(a)) | Some(Expected::Output(a)) => a.trim(),
                    None => "",
                };
                msg += &format!("\n      expected: {exp}\n      got:      {shown}");
            }
            Status::NoData => msg += &format!("\n      output: {shown}"),
            Status::RuntimeError | Status::Timeout => {
                let tail = out.stderr_tail(15);
                if !tail.is_empty() {
                    msg += &format!("\n{}", indent(&tail));
                }
            }
            _ => {}
        }
    }
    println!("{msg}");
}

/// `aoc run`: build and run one solution, printing its raw output.
pub fn run_one(
    ctx: &Ctx,
    engine: &Engine,
    impls: &[Impl],
    sol: (u16, Solution),
    imp_filter: Option<&str>,
    input: Option<PathBuf>,
) -> Result<bool> {
    let (year, sol) = sol;
    let candidates: Vec<usize> = impls
        .iter()
        .enumerate()
        .filter(|(_, i)| {
            i.year == year && i.solutions.contains(&sol) && imp_filter.is_none_or(|f| f == i.id)
        })
        .map(|(n, _)| n)
        .collect();
    let i = match candidates.as_slice() {
        [] => bail!("no implementation has {year} {sol}"),
        [i] => *i,
        many => bail!(
            "several implementations have {year} {sol}; pick one with --impl ({})",
            many.iter()
                .map(|&i| impls[i].id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    let imp = &impls[i];
    let (bytes, expected) = match input.as_deref() {
        Some(p) if p.as_os_str() == "-" => {
            let mut b = Vec::new();
            std::io::stdin().read_to_end(&mut b)?;
            (b, None)
        }
        Some(p) => (
            std::fs::read(p).with_context(|| format!("reading {}", p.display()))?,
            None,
        ),
        None => match data::input(ctx, year, sol.day)? {
            Some(b) => (b, data::answers(ctx, year, sol.day)?.expected(sol.part)),
            None => bail!(
                "no input for {year}/{:02}: pass --input FILE or run `cargo aoc fetch {year} {}`",
                sol.day,
                sol.day
            ),
        },
    };
    let built = build(engine, impls, &vec![(i, vec![sol])], 1);
    if built.failed.contains_key(&(i, sol)) {
        return Ok(false);
    }
    let out = engine.exec(&Exec {
        imp,
        image: &built.images[&i],
        sol: Some(sol),
        template: &imp.manifest.run,
        phase: Phase::Run,
        stdin: Some(&bytes),
        timeout: imp.run_timeout(sol.day)?,
    })?;
    std::io::stderr().write_all(&out.stderr)?;
    std::io::stdout().write_all(&out.stdout)?;
    let verdict = match (&out.exit, &expected) {
        (Exit::Code(0), Some(e)) if e.matches(&out.stdout) => " · matches recorded answer ✅",
        (Exit::Code(0), Some(_)) => " · does not match recorded answer ❌",
        _ => "",
    };
    eprintln!(
        "[{} {sol}: {} in {}{verdict}]",
        imp.name(),
        out.describe(),
        fmt_duration(out.elapsed)
    );
    Ok(out.ok())
}

/// `aoc list`
pub fn list(ctx: &Ctx, impls: &[Impl], work: &Work) {
    for (i, sols) in work {
        let imp = &impls[*i];
        let mut days: BTreeMap<u8, Vec<u8>> = BTreeMap::new();
        for s in sols {
            days.entry(s.day).or_default().push(s.part);
        }
        let days: Vec<String> = days
            .into_iter()
            .map(|(d, parts)| {
                if parts.len() == 2 {
                    format!("{d}")
                } else {
                    format!("{d}({})", parts[0])
                }
            })
            .collect();
        let toolchain = match (&imp.manifest.image, &imp.manifest.dockerfile) {
            (Some(img), _) => img.split("@sha256:").next().unwrap_or(img).to_string(),
            (_, Some(df)) => format!("{} ({df})", ctx.rel(&imp.dir).display()),
            _ => String::new(),
        };
        println!(
            "{:<20} {:<11} {:<28} days {}",
            imp.name(),
            imp.manifest.language,
            toolchain,
            if days.is_empty() {
                "-".to_string()
            } else {
                days.join(" ")
            }
        );
    }
}
