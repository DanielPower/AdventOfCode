//! `<year>/<impl>/aoc.toml`: how an implementation is built and run, and which
//! (day, part) solutions it contains.

use crate::model::{Solution, days_in_year, parts_for_day};
use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub const MANIFEST: &str = "aoc.toml";
const DEFAULT_RUN_TIMEOUT: Duration = Duration::from_secs(60);
pub const BUILD_TIMEOUT: Duration = Duration::from_secs(30 * 60);

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Language label used in reports.
    pub language: String,
    /// Toolchain image, pinned by digest (`name:tag@sha256:...`).
    pub image: Option<String>,
    /// Or: a Dockerfile (relative to the implementation) that builds the toolchain image.
    pub dockerfile: Option<String>,
    /// Force a platform, e.g. `linux/amd64` for images without an arm64 build.
    pub platform: Option<String>,
    /// Path pattern that identifies solutions, e.g. `{dd}/part{part}/Cargo.toml`.
    pub discover: Option<String>,
    /// Or: an explicit list of days (each with the year's default parts).
    pub days: Option<Vec<u8>>,
    /// Build command; runs once per distinct expansion (so per solution if it
    /// mentions {day}/{dd}/{part}, otherwise once per implementation).
    pub build: Option<String>,
    /// Run command; the puzzle input arrives on stdin.
    pub run: String,
    /// Path (template) whose files identify one solution's source, for cache
    /// keys. Defaults to the whole implementation directory.
    pub sources: Option<String>,
    /// Run timeout, e.g. "60s" or "5m".
    pub timeout: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Per-day overrides: `[day.12] timeout = "5m"`.
    #[serde(default)]
    pub day: BTreeMap<String, DayConfig>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct DayConfig {
    pub timeout: Option<String>,
    /// Parts present for this day when the discover pattern has no {part}.
    pub parts: Option<Vec<u8>>,
}

pub struct Impl {
    pub year: u16,
    pub id: String,
    /// Absolute path of the implementation directory.
    pub dir: PathBuf,
    pub manifest: Manifest,
    /// Raw manifest text (part of the cache key).
    pub manifest_text: String,
    pub solutions: BTreeSet<Solution>,
}

impl Impl {
    /// `2025/rust`
    pub fn name(&self) -> String {
        format!("{}/{}", self.year, self.id)
    }

    fn day_config(&self, day: u8) -> Option<&DayConfig> {
        self.manifest.day.get(&day.to_string())
    }

    pub fn run_timeout(&self, day: u8) -> Result<Duration> {
        let t = self
            .day_config(day)
            .and_then(|d| d.timeout.as_deref())
            .or(self.manifest.timeout.as_deref());
        match t {
            Some(t) => humantime::parse_duration(t)
                .with_context(|| format!("{}: bad timeout {t:?}", self.name())),
            None => Ok(DEFAULT_RUN_TIMEOUT),
        }
    }

    pub fn build_is_per_solution(&self) -> bool {
        self.manifest
            .build
            .as_deref()
            .is_some_and(has_solution_placeholder)
    }

    /// Substitute `{year} {day} {dd} {part} {cache} {src}` in a command template.
    pub fn expand(&self, template: &str, sol: Option<Solution>, cache: &str, src: &str) -> String {
        let mut s = template
            .replace("{year}", &self.year.to_string())
            .replace("{cache}", cache)
            .replace("{src}", src);
        if let Some(sol) = sol {
            s = s
                .replace("{day}", &sol.day.to_string())
                .replace("{dd}", &format!("{:02}", sol.day))
                .replace("{part}", &sol.part.to_string());
        }
        s
    }

    /// The source path for one solution, relative to the implementation dir.
    pub fn sources_path(&self, sol: Solution) -> PathBuf {
        match &self.manifest.sources {
            Some(t) => self.dir.join(self.expand(t, Some(sol), "", "")),
            None => self.dir.clone(),
        }
    }

    /// Text of the toolchain definition (part of the cache key).
    pub fn toolchain_fingerprint(&self) -> String {
        match (&self.manifest.image, &self.manifest.dockerfile) {
            (Some(image), _) => image.clone(),
            (_, Some(df)) => std::fs::read_to_string(self.dir.join(df)).unwrap_or_default(),
            _ => String::new(),
        }
    }
}

fn has_solution_placeholder(s: &str) -> bool {
    ["{day}", "{dd}", "{part}"].iter().any(|p| s.contains(p))
}

/// Load every `<year>/<impl>/aoc.toml` under the repository root.
pub fn load_all(root: &Path) -> Result<Vec<Impl>> {
    let mut impls = Vec::new();
    let mut years: Vec<_> = std::fs::read_dir(root)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| Some((e.file_name().to_str()?.parse::<u16>().ok()?, e.path())))
        .filter(|(y, _)| *y >= 2015)
        .collect();
    years.sort();
    for (year, year_dir) in years {
        let mut dirs: Vec<_> = std::fs::read_dir(&year_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join(MANIFEST).is_file())
            .collect();
        dirs.sort();
        for dir in dirs {
            impls.push(
                load_impl(year, &dir)
                    .with_context(|| format!("loading {}", dir.join(MANIFEST).display()))?,
            );
        }
    }
    Ok(impls)
}

pub fn load_impl(year: u16, dir: &Path) -> Result<Impl> {
    let manifest_text = std::fs::read_to_string(dir.join(MANIFEST))?;
    let manifest: Manifest = toml::from_str(&manifest_text)?;
    let id = dir
        .file_name()
        .and_then(|n| n.to_str())
        .context("bad directory name")?
        .to_string();
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        bail!("implementation directory {id:?} must be [A-Za-z0-9._-]");
    }
    match (&manifest.image, &manifest.dockerfile) {
        (Some(_), Some(_)) => bail!("set only one of `image` and `dockerfile`"),
        (None, None) => bail!("set `image` (pinned by digest) or `dockerfile`"),
        _ => {}
    }
    if let Some(df) = &manifest.dockerfile
        && !dir.join(df).is_file()
    {
        bail!("dockerfile {df:?} not found");
    }
    if let Some(t) = &manifest.timeout {
        humantime::parse_duration(t).with_context(|| format!("bad timeout {t:?}"))?;
    }
    for (day, cfg) in &manifest.day {
        let d: u8 = day
            .parse()
            .with_context(|| format!("[day.{day}]: not a day number"))?;
        if !(1..=days_in_year(year)).contains(&d) {
            bail!("[day.{day}]: {year} has days 1-{}", days_in_year(year));
        }
        if let Some(t) = &cfg.timeout {
            humantime::parse_duration(t)
                .with_context(|| format!("[day.{day}]: bad timeout {t:?}"))?;
        }
    }
    let mut imp = Impl {
        year,
        id,
        dir: dir.to_path_buf(),
        manifest,
        manifest_text,
        solutions: BTreeSet::new(),
    };
    imp.solutions = discover(&imp)?;
    Ok(imp)
}

fn parts_for(imp: &Impl, day: u8) -> Vec<u8> {
    imp.day_config(day)
        .and_then(|c| c.parts.clone())
        .unwrap_or_else(|| parts_for_day(imp.year, day).to_vec())
}

fn discover(imp: &Impl) -> Result<BTreeSet<Solution>> {
    let max_day = days_in_year(imp.year);
    let mut found = BTreeSet::new();
    match (&imp.manifest.discover, &imp.manifest.days) {
        (Some(_), Some(_)) => bail!("set only one of `discover` and `days`"),
        (None, None) => bail!("set `discover` (a path pattern) or `days` (a list)"),
        (None, Some(days)) => {
            for &day in days {
                if !(1..=max_day).contains(&day) {
                    bail!("`days`: {} has days 1-{max_day}", imp.year);
                }
                found.extend(
                    parts_for(imp, day)
                        .into_iter()
                        .map(|part| Solution { day, part }),
                );
            }
        }
        (Some(pattern), None) => {
            let (re, want_dir) = pattern_regex(pattern)?;
            let walker = ignore::WalkBuilder::new(&imp.dir)
                .require_git(false)
                .build();
            for entry in walker {
                let entry = entry?;
                let is_dir = entry.file_type().is_some_and(|t| t.is_dir());
                if is_dir != want_dir {
                    continue;
                }
                let Ok(rel) = entry.path().strip_prefix(&imp.dir) else {
                    continue;
                };
                let rel = rel.to_string_lossy().replace('\\', "/");
                let Some(caps) = re.captures(&rel) else {
                    continue;
                };
                let day: u8 = match caps
                    .name("day")
                    .or(caps.name("dd"))
                    .map(|m| m.as_str().parse())
                {
                    Some(Ok(d)) => d,
                    _ => continue,
                };
                if !(1..=max_day).contains(&day) {
                    continue;
                }
                match caps.name("part") {
                    Some(p) => {
                        let part = p.as_str().parse()?;
                        if parts_for_day(imp.year, day).contains(&part) {
                            found.insert(Solution { day, part });
                        }
                    }
                    None => found.extend(
                        parts_for(imp, day)
                            .into_iter()
                            .map(|part| Solution { day, part }),
                    ),
                }
            }
        }
    }
    Ok(found)
}

/// Turn `{dd}/part{part}/Cargo.toml` into an anchored regex. A trailing `/`
/// means the pattern matches directories.
pub fn pattern_regex(pattern: &str) -> Result<(Regex, bool)> {
    let want_dir = pattern.ends_with('/');
    let pattern = pattern.trim_end_matches('/');
    let placeholder = Regex::new(r"\{(day|dd|part)\}").unwrap();
    let mut re = String::from("^");
    let mut last = 0;
    let mut seen = BTreeSet::new();
    for m in placeholder.captures_iter(pattern) {
        let whole = m.get(0).unwrap();
        re.push_str(&regex::escape(&pattern[last..whole.start()]));
        let name = &m[1];
        let body = match name {
            "day" => "[0-9]{1,2}",
            "dd" => "[0-9]{2}",
            _ => "[12]",
        };
        if seen.insert(name.to_string()) {
            re.push_str(&format!("(?P<{name}>{body})"));
        } else {
            re.push_str(&format!("(?:{body})"));
        }
        last = whole.end();
    }
    re.push_str(&regex::escape(&pattern[last..]));
    re.push('$');
    if !seen.contains("day") && !seen.contains("dd") {
        bail!("discover pattern {pattern:?} needs {{day}} or {{dd}}");
    }
    Ok((Regex::new(&re)?, want_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns() {
        let (re, dir) = pattern_regex("{dd}/part{part}/Cargo.toml").unwrap();
        assert!(!dir);
        let c = re.captures("07/part2/Cargo.toml").unwrap();
        assert_eq!((&c["dd"], &c["part"]), ("07", "2"));
        assert!(re.captures("7/part2/Cargo.toml").is_none());

        let (re, dir) = pattern_regex("day{day}/").unwrap();
        assert!(dir);
        assert_eq!(&re.captures("day7").unwrap()["day"], "7");

        let (re, _) = pattern_regex("src/Day{day}.hs").unwrap();
        assert!(re.captures("src/DayX.hs").is_none());
        assert!(pattern_regex("src/part{part}.hs").is_err());
    }
}
