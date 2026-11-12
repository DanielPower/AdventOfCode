//! `results.json`: verified status per solution (no answers, no input), keyed
//! so unchanged solutions can be skipped and the README rebuilt offline.

use crate::manifest::Impl;
use crate::model::Solution;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

pub const FILE: &str = "results.json";

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Pass,
    Fail,
    BuildError,
    RuntimeError,
    Timeout,
    /// No input or no recorded answer to check against.
    NoData,
}

impl Status {
    pub fn is_failure(self) -> bool {
        matches!(
            self,
            Status::Fail | Status::BuildError | Status::RuntimeError | Status::Timeout
        )
    }

    pub fn icon(self) -> &'static str {
        match self {
            Status::Pass => "✅",
            Status::NoData => "❔",
            _ => "❌",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms: Option<u64>,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    pub version: u32,
    pub results: BTreeMap<String, Entry>,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            version: 1,
            results: BTreeMap::new(),
        }
    }
}

pub fn id(imp: &Impl, sol: Solution) -> String {
    format!("{}/{}/{:02}/{}", imp.year, imp.id, sol.day, sol.part)
}

impl Store {
    pub fn load(root: &Path) -> Result<Store> {
        let path = root.join(FILE);
        match std::fs::read_to_string(&path) {
            Ok(t) => {
                serde_json::from_str(&t).with_context(|| format!("parsing {}", path.display()))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Store::default()),
            Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }

    pub fn save(&self, root: &Path) -> Result<()> {
        let mut text = serde_json::to_string_pretty(self)?;
        text.push('\n');
        std::fs::write(root.join(FILE), text)?;
        Ok(())
    }

    /// Drop entries for solutions that no longer exist.
    pub fn prune(&mut self, impls: &[Impl]) {
        let live: std::collections::HashSet<String> = impls
            .iter()
            .flat_map(|i| i.solutions.iter().map(move |&s| id(i, s)))
            .collect();
        self.results.retain(|k, _| live.contains(k));
    }

    pub fn get(&self, imp: &Impl, sol: Solution) -> Option<&Entry> {
        self.results.get(&id(imp, sol))
    }
}

/// Computes cache keys: everything that can change a solution's result.
#[derive(Default)]
pub struct Keyer {
    trees: HashMap<PathBuf, String>,
}

impl Keyer {
    pub fn key(&mut self, imp: &Impl, sol: Solution, input: &[u8], expected: &str) -> String {
        let src = imp.sources_path(sol);
        let tree = self
            .trees
            .entry(src.clone())
            .or_insert_with(|| hash_tree(&src))
            .clone();
        let mut h = Sha256::new();
        for part in [
            b"aoc-key-v1".as_slice(),
            imp.manifest_text.as_bytes(),
            imp.toolchain_fingerprint().as_bytes(),
            tree.as_bytes(),
            &Sha256::digest(input),
            expected.as_bytes(),
        ] {
            h.update((part.len() as u64).to_le_bytes());
            h.update(part);
        }
        hex(&h.finalize())
    }
}

/// Hash of every non-ignored file under `dir` (respects .gitignore, skips hidden files).
pub fn hash_tree(dir: &Path) -> String {
    let mut files: Vec<PathBuf> = ignore::WalkBuilder::new(dir)
        .require_git(false)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
        .map(|e| e.into_path())
        .collect();
    files.sort();
    let mut h = Sha256::new();
    for f in files {
        let rel = f
            .strip_prefix(dir)
            .unwrap_or(&f)
            .to_string_lossy()
            .replace('\\', "/");
        let content = std::fs::read(&f).unwrap_or_default();
        h.update((rel.len() as u64).to_le_bytes());
        h.update(rel.as_bytes());
        h.update((content.len() as u64).to_le_bytes());
        h.update(&content);
    }
    hex(&h.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
