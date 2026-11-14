use anyhow::{Result, bail};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

pub struct Ctx {
    pub root: PathBuf,
    pub data_dir: PathBuf,
    /// Per-implementation build caches, mounted at `/cache` in containers.
    pub cache_dir: PathBuf,
    pub runtime: String,
    /// Optional CA bundle for builds behind a TLS-intercepting proxy.
    pub extra_ca: Option<PathBuf>,
    dotenv: HashMap<String, String>,
}

impl Ctx {
    pub fn load() -> Result<Ctx> {
        let root = find_root()?;
        let dotenv = read_dotenv(&root.join(".env"));
        let mut ctx = Ctx {
            data_dir: root.join(".aoc-data"),
            cache_dir: root.join(".aoc-cache"),
            runtime: "docker".into(),
            extra_ca: None,
            root,
            dotenv,
        };
        if let Some(p) = ctx.var("AOC_DATA_DIR") {
            ctx.data_dir = ctx.resolve(&p);
        }
        if let Some(p) = ctx.var("AOC_CACHE_DIR") {
            ctx.cache_dir = ctx.resolve(&p);
        }
        if let Some(r) = ctx.var("AOC_CONTAINER_RUNTIME") {
            ctx.runtime = r;
        }
        if let Some(p) = ctx.var("AOC_EXTRA_CA") {
            ctx.extra_ca = Some(ctx.resolve(&p));
        }
        Ok(ctx)
    }

    /// Environment variable, falling back to the repository's (gitignored) `.env` file.
    pub fn var(&self, name: &str) -> Option<String> {
        env::var(name)
            .ok()
            .or_else(|| self.dotenv.get(name).cloned())
            .filter(|v| !v.is_empty())
    }

    fn resolve(&self, p: &str) -> PathBuf {
        let p = match p.strip_prefix("~/") {
            Some(rest) => env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_default()
                .join(rest),
            None => PathBuf::from(p),
        };
        if p.is_absolute() {
            p
        } else {
            self.root.join(p)
        }
    }

    pub fn rel<'a>(&self, p: &'a Path) -> &'a Path {
        p.strip_prefix(&self.root).unwrap_or(p)
    }
}

fn find_root() -> Result<PathBuf> {
    if let Some(r) = env::var_os("AOC_ROOT") {
        return Ok(PathBuf::from(r));
    }
    let mut dir = env::current_dir()?;
    loop {
        if dir.join("tools/aoc/Cargo.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    // Fall back to the checkout this binary was built from.
    let built = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if built.join("tools/aoc/Cargo.toml").is_file() {
        return Ok(built.canonicalize()?);
    }
    bail!("could not find the repository root (a directory containing tools/aoc); set AOC_ROOT")
}

fn read_dotenv(path: &Path) -> HashMap<String, String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return HashMap::new();
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.strip_prefix("export ").unwrap_or(l).split_once('='))
        .map(|(k, v)| {
            let v = v.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .or_else(|| v.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                .unwrap_or(v);
            (k.trim().to_string(), v.to_string())
        })
        .collect()
}
