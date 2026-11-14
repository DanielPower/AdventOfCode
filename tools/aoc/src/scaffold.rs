//! `aoc new`: scaffold a solution from an implementation's `[new]` section:
//! copy its template directory (expanding placeholders), then run its command.

use crate::ctx::Ctx;
use crate::data;
use crate::engine::{Engine, Exec, Phase};
use crate::manifest::{self, BUILD_TIMEOUT, Impl};
use crate::model::{Solution, days_in_year, parts_for_day};
use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Expand `{{year}} {{day}} {{dd}} {{part}}` in a template file's name or
/// contents. (Double braces, so code like Rust's `println!("{day}")` or
/// Python's `f"{day}"` is left alone.)
pub fn expand_template(s: &str, year: u16, sol: Solution) -> String {
    s.replace("{{year}}", &year.to_string())
        .replace("{{dd}}", &format!("{:02}", sol.day))
        .replace("{{day}}", &sol.day.to_string())
        .replace("{{part}}", &sol.part.to_string())
}

pub struct NewOpts<'a> {
    pub year: u16,
    pub day: u8,
    pub part: Option<u8>,
    pub imp: Option<&'a str>,
    pub fetch: bool,
}

pub fn new_solution(ctx: &Ctx, engine: &Engine, impls: &[Impl], o: &NewOpts) -> Result<bool> {
    let (year, day) = (o.year, o.day);
    let max = days_in_year(year);
    if !(1..=max).contains(&day) {
        bail!("{year} has days 1-{max}");
    }
    let part = o.part.unwrap_or(1);
    if !parts_for_day(year, day).contains(&part) {
        bail!("{year} day {day} has no part {part}");
    }
    let sol = Solution { day, part };

    let candidates: Vec<&Impl> = impls
        .iter()
        .filter(|i| i.year == year && o.imp.is_none_or(|f| f == i.id))
        .collect();
    let imp = match candidates.as_slice() {
        [] => match o.imp {
            Some(f) => bail!("no implementation {year}/{f}: create {year}/{f}/aoc.toml first"),
            None => bail!("{year} has no implementations yet: create {year}/<name>/aoc.toml first"),
        },
        [one] => *one,
        many => bail!(
            "{year} has several implementations; pick one with --impl ({})",
            many.iter()
                .map(|i| i.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    let Some(cfg) = &imp.manifest.new else {
        bail!(
            "{} has no [new] section in its aoc.toml, so there's nothing to scaffold from",
            imp.name()
        );
    };
    if imp.solutions.contains(&sol) {
        bail!("{} already has {sol}", imp.name());
    }

    let mut copies = Vec::new();
    if let Some(t) = &cfg.template {
        let template_dir = imp.dir.join(t);
        for entry in ignore::WalkBuilder::new(&template_dir)
            .standard_filters(false)
            .build()
        {
            let entry = entry?;
            if !entry.file_type().is_some_and(|t| t.is_file()) {
                continue;
            }
            let rel = entry
                .path()
                .strip_prefix(&template_dir)?
                .to_string_lossy()
                .replace('\\', "/");
            copies.push((
                entry.path().to_path_buf(),
                imp.dir.join(expand_template(&rel, year, sol)),
            ));
        }
        if copies.is_empty() {
            bail!(
                "template directory {} is empty",
                ctx.rel(&template_dir).display()
            );
        }
    }
    let clashes: Vec<String> = copies
        .iter()
        .filter(|(_, dest)| dest.exists())
        .map(|(_, dest)| ctx.rel(dest).display().to_string())
        .collect();
    if !clashes.is_empty() {
        bail!(
            "refusing to overwrite existing files: {}",
            clashes.join(", ")
        );
    }

    // Get the toolchain first, so a missing image fails before anything is written.
    let image = match &cfg.command {
        Some(_) => Some(engine.prepare_image(imp)?),
        None => None,
    };

    let before = snapshot(&imp.dir);
    let result = (|| -> Result<bool> {
        for (src, dest) in &copies {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let bytes = std::fs::read(src).with_context(|| format!("reading {}", src.display()))?;
            match String::from_utf8(bytes) {
                Ok(text) => std::fs::write(dest, expand_template(&text, year, sol))?,
                Err(binary) => std::fs::write(dest, binary.into_bytes())?,
            }
            std::fs::set_permissions(dest, std::fs::metadata(src)?.permissions())?;
        }
        if let (Some(command), Some(image)) = (&cfg.command, &image) {
            let out = engine.exec(&Exec {
                imp,
                image,
                sol: Some(sol),
                template: command,
                phase: Phase::Build,
                stdin: None,
                timeout: BUILD_TIMEOUT,
            })?;
            if !out.ok() {
                eprintln!(
                    "❌ the [new] command failed ({}):\n{}",
                    out.describe(),
                    out.stderr_tail(30)
                );
                return Ok(false);
            }
        }
        Ok(true)
    })();
    if !matches!(result, Ok(true)) {
        // Leave nothing half-made behind, so the command can simply be re-run.
        let dests = copies.iter().map(|(_, dest)| dest.clone());
        rollback(
            &imp.dir,
            snapshot(&imp.dir).difference(&before).cloned().chain(dests),
        );
        eprintln!("removed the partially created files");
        return result;
    }

    println!("created {} {sol}:", imp.name());
    report_created(ctx, &before, &imp.dir);

    let reloaded = manifest::load_impl(year, &imp.dir)?;
    if !reloaded.solutions.contains(&sol) {
        eprintln!(
            "warning: `discover` doesn't match what [new] created, so aoc won't find {sol}; check {}",
            ctx.rel(&imp.dir.join(manifest::MANIFEST)).display()
        );
    }
    if o.fetch {
        fetch(ctx, year, day);
    } else if data::input(ctx, year, day)?.is_none() {
        println!("next: cargo aoc fetch {year} {day}");
    }
    println!(
        "then: cargo aoc test {year}/{day}/{part}   (or: cargo aoc run {year} {day} {part} --native)"
    );
    Ok(true)
}

/// Download the day's input and any answers already submitted (so scaffolding
/// part 2 also records part 1's answer). Problems here never undo the scaffold.
fn fetch(ctx: &Ctx, year: u16, day: u8) {
    if ctx.var("AOC_SESSION").is_none() {
        println!(
            "note: AOC_SESSION isn't set, so nothing was downloaded; set it in .env, then run `cargo aoc fetch {year} {day}`"
        );
        return;
    }
    if let Err(e) = data::fetch(ctx, year, Some(day), false) {
        eprintln!(
            "warning: couldn't fetch {year} day {day}: {e:#}\n(the scaffold is in place; retry with `cargo aoc fetch {year} {day}`)"
        );
    }
}

/// Files in the implementation (respecting .gitignore, so build output is skipped).
fn snapshot(dir: &Path) -> BTreeSet<PathBuf> {
    ignore::WalkBuilder::new(dir)
        .require_git(false)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
        .map(|e| e.into_path())
        .collect()
}

/// Delete files the scaffold created, then any directories left empty.
fn rollback(root: &Path, files: impl Iterator<Item = PathBuf>) {
    let mut dirs = BTreeSet::new();
    for f in files {
        let _ = std::fs::remove_file(&f);
        let mut dir = f.parent();
        while let Some(d) = dir.filter(|d| d.starts_with(root) && *d != root) {
            dirs.insert(d.to_path_buf());
            dir = d.parent();
        }
    }
    // Deepest first; remove_dir only succeeds on empty directories.
    for d in dirs.iter().rev() {
        let _ = std::fs::remove_dir(d);
    }
}

fn report_created(ctx: &Ctx, before: &BTreeSet<PathBuf>, dir: &Path) {
    for path in snapshot(dir).difference(before) {
        println!("  {}", ctx.rel(path).display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_double_braces_only() {
        let sol = Solution { day: 7, part: 2 };
        assert_eq!(
            expand_template("{{dd}}/part{{part}}/main.rs", 2025, sol),
            "07/part2/main.rs"
        );
        assert_eq!(
            expand_template("println!(\"{day} {{day}} {{year}}\")", 2025, sol),
            "println!(\"{day} 7 2025\")"
        );
    }
}
