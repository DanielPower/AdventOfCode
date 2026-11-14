//! The private data directory (never committed to this repository):
//!
//! ```text
//! <data>/<year>/<dd>/input.txt
//! <data>/<year>/<dd>/answers.toml     part1 = "1234"  part2 = "..."
//! <data>/<year>/<dd>/examples.toml    [[example]] input = "1.txt"  part1 = "..."
//! <data>/<year>/<dd>/1.txt            (example inputs)
//! ```

use crate::ctx::Ctx;
use crate::model::{days_in_year, parts_for_day};
use crate::sync;
use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Answers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part2: Option<String>,
    /// Exact expected stdout, for puzzles whose answer is read off ASCII art.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part1_output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part2_output: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Expected {
    /// Compared with the last non-empty line of stdout, so debug output
    /// printed before the answer is tolerated.
    Answer(String),
    /// Compared line by line, ignoring trailing whitespace.
    Output(String),
}

impl Answers {
    pub fn expected(&self, part: u8) -> Option<Expected> {
        let (answer, output) = match part {
            1 => (&self.part1, &self.part1_output),
            _ => (&self.part2, &self.part2_output),
        };
        output
            .clone()
            .map(Expected::Output)
            .or_else(|| answer.clone().map(Expected::Answer))
    }

    fn set(&mut self, part: u8, value: String) {
        match part {
            1 => self.part1 = Some(value),
            _ => self.part2 = Some(value),
        }
    }

    fn has(&self, part: u8) -> bool {
        self.expected(part).is_some()
    }
}

impl Expected {
    pub fn matches(&self, stdout: &[u8]) -> bool {
        let actual = String::from_utf8_lossy(stdout);
        match self {
            Expected::Answer(a) => {
                actual.lines().map(str::trim).rfind(|l| !l.is_empty()) == Some(a.trim())
            }
            Expected::Output(o) => normalize(&actual) == normalize(o),
        }
    }

    /// The strings to mask in CI logs.
    pub fn secrets(&self) -> Vec<String> {
        let s = match self {
            Expected::Answer(a) | Expected::Output(a) => a,
        };
        s.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect()
    }

    pub fn as_key(&self) -> String {
        match self {
            Expected::Answer(a) => format!("answer:{}", a.trim()),
            Expected::Output(o) => format!("output:{}", normalize(o)),
        }
    }
}

fn normalize(s: &str) -> String {
    s.lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_matches('\n')
        .to_string()
}

pub struct Example {
    pub name: String,
    pub input: Vec<u8>,
    pub answers: Answers,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExamplesFile {
    #[serde(default)]
    example: Vec<ExampleEntry>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleEntry {
    input: String,
    #[serde(flatten)]
    answers: Answers,
}

pub fn day_dir(ctx: &Ctx, year: u16, day: u8) -> PathBuf {
    ctx.data_dir
        .join(year.to_string())
        .join(format!("{day:02}"))
}

pub fn input(ctx: &Ctx, year: u16, day: u8) -> Result<Option<Vec<u8>>> {
    let path = day_dir(ctx, year, day).join("input.txt");
    match std::fs::read(&path) {
        Ok(b) => Ok(Some(b)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
    }
}

pub fn answers(ctx: &Ctx, year: u16, day: u8) -> Result<Answers> {
    read_answers(&day_dir(ctx, year, day).join("answers.toml"))
}

fn read_answers(path: &Path) -> Result<Answers> {
    match std::fs::read_to_string(path) {
        Ok(t) => toml::from_str(&t).with_context(|| format!("parsing {}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Answers::default()),
        Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
    }
}

fn write_answers(path: &Path, answers: &Answers) -> Result<()> {
    std::fs::write(path, toml::to_string(answers)?)
        .with_context(|| format!("writing {}", path.display()))
}

pub fn examples(ctx: &Ctx, year: u16, day: u8) -> Result<Vec<Example>> {
    let dir = day_dir(ctx, year, day);
    let path = dir.join("examples.toml");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
    };
    let file: ExamplesFile =
        toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    file.example
        .into_iter()
        .map(|e| {
            let input = std::fs::read(dir.join(&e.input))
                .with_context(|| format!("reading example {}", e.input))?;
            Ok(Example {
                name: e.input,
                input,
                answers: e.answers,
            })
        })
        .collect()
}

/// Import the old layout (`<year>/data/DD.in`, `<year>/data/DD.P.out`, and the
/// Haskell years' `<year>/inputs/DD.in`) into the data directory.
pub fn import(ctx: &Ctx, from: &Path, force: bool) -> Result<()> {
    let name_re = Regex::new(r"^(\d{2})\.(?:in|(\d)\.out)$").unwrap();
    let (mut inputs, mut answers, mut skipped) = (0, 0, 0);
    for year_entry in std::fs::read_dir(from)? {
        let year_entry = year_entry?;
        let Some(year) = year_entry
            .file_name()
            .to_str()
            .and_then(|n| n.parse::<u16>().ok())
        else {
            continue;
        };
        for sub in ["data", "inputs"] {
            let dir = year_entry.path().join(sub);
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries {
                let entry = entry?;
                let fname = entry.file_name().to_string_lossy().into_owned();
                let Some(caps) = name_re.captures(&fname) else {
                    eprintln!(
                        "skipping {} (not DD.in or DD.P.out)",
                        ctx.rel(&entry.path()).display()
                    );
                    continue;
                };
                let day: u8 = caps[1].parse()?;
                let target_dir = day_dir(ctx, year, day);
                std::fs::create_dir_all(&target_dir)?;
                match caps.get(2) {
                    None => {
                        let target = target_dir.join("input.txt");
                        if target.exists() && !force {
                            skipped += 1;
                            continue;
                        }
                        std::fs::copy(entry.path(), &target)?;
                        inputs += 1;
                    }
                    Some(p) => {
                        let part: u8 = p.as_str().parse()?;
                        let path = target_dir.join("answers.toml");
                        let mut a = read_answers(&path)?;
                        if a.has(part) && !force {
                            skipped += 1;
                            continue;
                        }
                        let text = std::fs::read_to_string(entry.path())?;
                        let value = text.trim().to_string();
                        if value.contains('\n') {
                            match part {
                                1 => a.part1_output = Some(value),
                                _ => a.part2_output = Some(value),
                            }
                        } else {
                            a.set(part, value);
                        }
                        write_answers(&path, &a)?;
                        answers += 1;
                    }
                }
            }
        }
    }
    eprintln!(
        "imported {inputs} inputs and {answers} answers into {} ({skipped} already present; --force to overwrite)",
        ctx.data_dir.display()
    );
    sync::save(ctx);
    Ok(())
}

/// Download inputs and your already-submitted answers from adventofcode.com.
pub fn fetch(ctx: &Ctx, year: u16, day: Option<u8>, force: bool) -> Result<()> {
    let max = days_in_year(year);
    let days: Vec<(u16, u8)> = match day {
        Some(d) if (1..=max).contains(&d) => vec![(year, d)],
        Some(_) => bail!("{year} has days 1-{max}"),
        None => (1..=max).map(|d| (year, d)).collect(),
    };
    fetch_days(ctx, &days, force)
}

/// Fetch several days, bracketed by one pull and one commit+push of the data
/// repository. Whatever was downloaded is saved even if a later request fails.
pub fn fetch_days(ctx: &Ctx, days: &[(u16, u8)], force: bool) -> Result<()> {
    let session = ctx.var("AOC_SESSION").context(
        "AOC_SESSION is not set: copy the `session` cookie from adventofcode.com into the environment or .env",
    )?;
    let agent = ctx.var("AOC_USER_AGENT").unwrap_or_else(|| {
        eprintln!("note: set AOC_USER_AGENT to something that identifies you (AoC asks automated tools to include contact info)");
        format!("aoc-runner/{}", env!("CARGO_PKG_VERSION"))
    });
    let mut client = Client {
        session,
        agent,
        last: None,
    };
    // Pull first: another machine may already have downloaded these.
    sync::pull(ctx);
    let result = days
        .iter()
        .try_for_each(|&(year, day)| fetch_day(ctx, &mut client, year, day, force));
    sync::save(ctx);
    result
}

fn fetch_day(ctx: &Ctx, client: &mut Client, year: u16, day: u8, force: bool) -> Result<()> {
    if !unlocked(year, day) {
        eprintln!("{year}/{day:02}: not unlocked yet");
        return Ok(());
    }
    let dir = day_dir(ctx, year, day);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;

    let input_path = dir.join("input.txt");
    if force || !input_path.exists() {
        match client.get(&format!("https://adventofcode.com/{year}/day/{day}/input"))? {
            Some(body) => {
                std::fs::write(&input_path, body)?;
                eprintln!("{year}/{day:02}: saved input");
            }
            None => eprintln!("{year}/{day:02}: no input available"),
        }
    }

    let answers_path = dir.join("answers.toml");
    let mut answers = read_answers(&answers_path)?;
    let parts = parts_for_day(year, day);
    if force || parts.iter().any(|&p| !answers.has(p)) {
        let Some(page) = client.get(&format!("https://adventofcode.com/{year}/day/{day}"))? else {
            return Ok(());
        };
        let found = scrape_answers(&page);
        let mut saved = 0;
        for (&part, value) in parts.iter().zip(found) {
            if force || !answers.has(part) {
                answers.set(part, value);
                saved += 1;
            }
        }
        if saved > 0 {
            write_answers(&answers_path, &answers)?;
            eprintln!("{year}/{day:02}: saved {saved} answer(s)");
        }
    }
    Ok(())
}

struct Client {
    session: String,
    agent: String,
    last: Option<Instant>,
}

impl Client {
    /// GET with the session cookie, at most one request every 3 seconds.
    /// Returns None for 404 (puzzle missing or not unlocked).
    fn get(&mut self, url: &str) -> Result<Option<String>> {
        if let Some(last) = self.last {
            let gap = Duration::from_secs(3);
            if last.elapsed() < gap {
                std::thread::sleep(gap - last.elapsed());
            }
        }
        self.last = Some(Instant::now());
        match ureq::get(url)
            .set("Cookie", &format!("session={}", self.session))
            .set("User-Agent", &self.agent)
            .call()
        {
            Ok(resp) => Ok(Some(resp.into_string()?)),
            Err(ureq::Error::Status(404, _)) => Ok(None),
            Err(ureq::Error::Status(400 | 500, _)) => {
                bail!(
                    "{url}: the server rejected the session cookie (expired? refresh AOC_SESSION)"
                )
            }
            Err(e) => Err(e).with_context(|| format!("fetching {url}")),
        }
    }
}

/// Answers shown on a puzzle page once solved, in part order.
pub fn scrape_answers(html: &str) -> Vec<String> {
    let re = Regex::new(r"Your puzzle answer was <code>([^<]*)</code>").unwrap();
    re.captures_iter(html)
        .map(|c| {
            c[1].replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .replace("&amp;", "&")
        })
        .collect()
}

/// Puzzles unlock at midnight EST (05:00 UTC) on December `day`.
pub fn unlocked(year: u16, day: u8) -> bool {
    let unlock = days_from_civil(year as i64, 12, day as i64) * 86_400 + 5 * 3600;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    now >= unlock
}

/// Days since 1970-01-01 (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrapes_answers() {
        let html = r#"<p>Your puzzle answer was <code>1234</code>.</p><article>...</article>
            <p>Your puzzle answer was <code>A&lt;B</code>.</p>"#;
        assert_eq!(scrape_answers(html), vec!["1234", "A<B"]);
    }

    #[test]
    fn compares() {
        assert!(Expected::Answer("42".into()).matches(b"42\n"));
        assert!(!Expected::Answer("42".into()).matches(b"420\n"));
        assert!(Expected::Answer("42".into()).matches(b"debug: 1\ndebug: 2\n42\n\n"));
        assert!(!Expected::Answer("42".into()).matches(b"42\ndebug after\n"));
        assert!(Expected::Output("#..#\n#..#".into()).matches(b"\n#..#  \n#..#\n"));
    }

    #[test]
    fn civil() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2024, 12, 1), 20058);
    }
}
