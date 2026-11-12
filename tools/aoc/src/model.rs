//! Calendar rules and the `YEAR[/DAY[/PART]]` selector.

use anyhow::{Context, Result, bail};
use std::fmt;
use std::str::FromStr;

/// Number of puzzles in a year: 25 until 2024, 12 from 2025 on.
pub fn days_in_year(year: u16) -> u8 {
    if year >= 2025 { 12 } else { 25 }
}

/// The final puzzle of each year has a single part.
pub fn parts_for_day(year: u16, day: u8) -> &'static [u8] {
    if day == days_in_year(year) {
        &[1]
    } else {
        &[1, 2]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Solution {
    pub day: u8,
    pub part: u8,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{}", self.day, self.part)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Selector {
    pub year: Option<u16>,
    pub day: Option<u8>,
    pub part: Option<u8>,
}

impl Selector {
    pub fn matches_year(&self, year: u16) -> bool {
        self.year.is_none_or(|y| y == year)
    }

    pub fn matches(&self, year: u16, sol: Solution) -> bool {
        self.matches_year(year)
            && self.day.is_none_or(|d| d == sol.day)
            && self.part.is_none_or(|p| p == sol.part)
    }
}

impl FromStr for Selector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut it = s.split('/').filter(|p| !p.is_empty());
        let mut sel = Selector::default();
        if let Some(y) = it.next() {
            sel.year = Some(y.parse().with_context(|| format!("bad year {y:?}"))?);
        }
        if let Some(d) = it.next() {
            sel.day = Some(d.parse().with_context(|| format!("bad day {d:?}"))?);
        }
        if let Some(p) = it.next() {
            let p: u8 = p.parse().with_context(|| format!("bad part {p:?}"))?;
            if !(1..=2).contains(&p) {
                bail!("part must be 1 or 2");
            }
            sel.part = Some(p);
        }
        if it.next().is_some() {
            bail!("expected YEAR, YEAR/DAY or YEAR/DAY/PART");
        }
        Ok(sel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_parses() {
        let s: Selector = "2025/7/2".parse().unwrap();
        assert_eq!((s.year, s.day, s.part), (Some(2025), Some(7), Some(2)));
        assert!(s.matches(2025, Solution { day: 7, part: 2 }));
        assert!(!s.matches(2025, Solution { day: 7, part: 1 }));
        assert!("2025/7/3".parse::<Selector>().is_err());
    }

    #[test]
    fn calendar() {
        assert_eq!(days_in_year(2024), 25);
        assert_eq!(days_in_year(2025), 12);
        assert_eq!(parts_for_day(2025, 12), &[1]);
        assert_eq!(parts_for_day(2024, 24), &[1, 2]);
    }
}
