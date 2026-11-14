//! `aoc`: build, run and verify Advent of Code solutions in pinned containers.
//!
//! Run it from anywhere in the repository with `cargo aoc <command>` (an alias
//! defined in `.cargo/config.toml`).

mod ctx;
mod data;
mod engine;
mod manifest;
mod model;
mod pipeline;
mod pool;
mod readme;
mod results;
mod scaffold;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use ctx::Ctx;
use engine::Engine;
use model::{Selector, Solution};
use pipeline::{SelectOpts, TestOpts};
use std::path::PathBuf;
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(
    name = "aoc",
    version,
    about = "Build, run and verify Advent of Code solutions in pinned containers"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List implementations and the solutions they contain
    List(SelectArgs),
    /// Scaffold a new solution from the implementation's [new] template/command
    New {
        year: u16,
        day: u8,
        /// Part to scaffold, for implementations with one project per part (default: 1)
        part: Option<u8>,
        /// Implementation, when the year has several
        #[arg(long = "impl")]
        imp: Option<String>,
        /// Run the [new] command with the host's toolchain instead of the container
        #[arg(long)]
        native: bool,
    },
    /// Build solutions (needs no puzzle data)
    Build {
        #[command(flatten)]
        select: SelectArgs,
        #[command(flatten)]
        exec: ExecArgs,
    },
    /// Build and run one solution, printing its output
    Run {
        year: u16,
        day: u8,
        part: u8,
        /// Implementation to use when several have this solution
        #[arg(long = "impl")]
        imp: Option<String>,
        /// Input file, or `-` for stdin (default: the input in the data directory)
        #[arg(long)]
        input: Option<PathBuf>,
        /// Use the host's toolchain instead of the container (fast, not reproducible)
        #[arg(long)]
        native: bool,
    },
    /// Build, run and check solutions against the recorded answers
    Test {
        #[command(flatten)]
        select: SelectArgs,
        #[command(flatten)]
        exec: ExecArgs,
        /// Check against the example inputs (examples.toml) instead
        #[arg(long)]
        examples: bool,
        /// Skip solutions whose results.json entry is still current
        #[arg(long)]
        changed: bool,
        /// Record results in results.json (what CI does)
        #[arg(long)]
        save: bool,
        /// Also write this run's results (status and timing only) as JSON
        #[arg(long, value_name = "FILE")]
        json: Option<PathBuf>,
        /// CI mode: never print answers, outputs or stderr; mask answers in GitHub logs
        #[arg(long)]
        ci: bool,
    },
    /// Download inputs and your submitted answers into the data directory
    Fetch {
        year: u16,
        day: Option<u8>,
        /// Re-download even if present
        #[arg(long)]
        force: bool,
    },
    /// Manage the private data directory
    #[command(subcommand)]
    Data(DataCmd),
    /// Regenerate the progress block in README.md from results.json
    Readme {
        /// Exit with an error if README.md is out of date instead of writing it
        #[arg(long)]
        check: bool,
    },
    /// Manage toolchain images
    #[command(subcommand)]
    Toolchain(ToolchainCmd),
    /// Check that everything needed is set up
    Doctor,
}

#[derive(Subcommand)]
enum DataCmd {
    /// Copy old-layout files (<year>/data/DD.in, DD.P.out, <year>/inputs/DD.in) into the data directory
    Import {
        /// Directory containing the <year> folders (default: repository root)
        #[arg(long)]
        from: Option<PathBuf>,
        #[arg(long)]
        force: bool,
    },
    /// Print the data directory's path
    Path,
}

#[derive(Subcommand)]
enum ToolchainCmd {
    /// Resolve an implementation's image tag to a digest and write it to aoc.toml
    Pin {
        /// Implementation, e.g. 2025/rust
        name: String,
        /// Re-resolve the tag even if already pinned
        #[arg(long)]
        update: bool,
    },
}

#[derive(Args)]
struct SelectArgs {
    /// YEAR, YEAR/DAY or YEAR/DAY/PART (default: everything)
    selector: Option<Selector>,
    /// Only this implementation (its directory name, e.g. rust)
    #[arg(long = "impl")]
    imp: Option<String>,
    /// Only implementations with files changed since this git ref
    #[arg(long, value_name = "REF")]
    since: Option<String>,
}

#[derive(Args)]
struct ExecArgs {
    /// Parallel builds/runs (default: number of CPUs)
    #[arg(long, short)]
    jobs: Option<usize>,
    /// Use the host's toolchain instead of containers (fast, not reproducible)
    #[arg(long)]
    native: bool,
}

impl SelectArgs {
    fn opts(&self) -> SelectOpts {
        SelectOpts {
            sel: self.selector.unwrap_or_default(),
            imp: self.imp.clone(),
            since: self.since.clone(),
            include_empty: false,
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool> {
    let ctx = Ctx::load()?;
    match cli.cmd {
        Cmd::List(select) => {
            let impls = manifest::load_all(&ctx.root)?;
            let opts = SelectOpts {
                include_empty: select.selector.is_none_or(|s| s.day.is_none()),
                ..select.opts()
            };
            let work = pipeline::select(&ctx, &impls, &opts)?;
            pipeline::list(&ctx, &impls, &work);
            Ok(true)
        }
        Cmd::New {
            year,
            day,
            part,
            imp,
            native,
        } => {
            let impls = manifest::load_all(&ctx.root)?;
            let engine = Engine { ctx: &ctx, native };
            let opts = scaffold::NewOpts {
                year,
                day,
                part,
                imp: imp.as_deref(),
            };
            scaffold::new_solution(&ctx, &engine, &impls, &opts)
        }
        Cmd::Build { select, exec } => {
            let impls = manifest::load_all(&ctx.root)?;
            let work = pipeline::select(&ctx, &impls, &select.opts())?;
            if work.is_empty() {
                println!("nothing selected");
                return Ok(true);
            }
            let engine = Engine {
                ctx: &ctx,
                native: exec.native,
            };
            engine.cleanup_stale();
            let built = pipeline::build(
                &engine,
                &impls,
                &work,
                exec.jobs.unwrap_or_else(pool::default_jobs),
            );
            Ok(built.failed.is_empty())
        }
        Cmd::Run {
            year,
            day,
            part,
            imp,
            input,
            native,
        } => {
            let impls = manifest::load_all(&ctx.root)?;
            let engine = Engine { ctx: &ctx, native };
            engine.cleanup_stale();
            pipeline::run_one(
                &ctx,
                &engine,
                &impls,
                (year, Solution { day, part }),
                imp.as_deref(),
                input,
            )
        }
        Cmd::Test {
            select,
            exec,
            examples,
            changed,
            save,
            json,
            ci,
        } => {
            let impls = manifest::load_all(&ctx.root)?;
            let work = pipeline::select(&ctx, &impls, &select.opts())?;
            let engine = Engine {
                ctx: &ctx,
                native: exec.native,
            };
            engine.cleanup_stale();
            let opts = TestOpts {
                jobs: exec.jobs.unwrap_or_else(pool::default_jobs),
                ci,
                changed,
                examples,
                save,
                json,
            };
            pipeline::test(&ctx, &engine, &impls, work, &opts)
        }
        Cmd::Fetch { year, day, force } => data::fetch(&ctx, year, day, force).map(|_| true),
        Cmd::Data(DataCmd::Import { from, force }) => {
            data::import(&ctx, &from.unwrap_or_else(|| ctx.root.clone()), force).map(|_| true)
        }
        Cmd::Data(DataCmd::Path) => {
            println!("{}", ctx.data_dir.display());
            Ok(true)
        }
        Cmd::Readme { check } => {
            let impls = manifest::load_all(&ctx.root)?;
            let store = results::Store::load(&ctx.root)?;
            let changed = readme::update(&ctx.root, &impls, &store, check)?;
            match (check, changed) {
                (true, true) => {
                    eprintln!("README.md is out of date; run `cargo aoc readme`");
                    Ok(false)
                }
                (false, true) => {
                    eprintln!("updated README.md");
                    Ok(true)
                }
                _ => Ok(true),
            }
        }
        Cmd::Toolchain(ToolchainCmd::Pin { name, update }) => {
            pin(&ctx, &name, update).map(|_| true)
        }
        Cmd::Doctor => doctor(&ctx),
    }
}

fn pin(ctx: &Ctx, name: &str, update: bool) -> Result<()> {
    let impls = manifest::load_all(&ctx.root)?;
    let imp = impls
        .iter()
        .find(|i| i.name() == name)
        .with_context(|| format!("no implementation {name:?}"))?;
    let Some(image) = &imp.manifest.image else {
        bail!(
            "{name} builds its toolchain from a Dockerfile; pin the digests on its FROM lines instead"
        );
    };
    if image.contains("@sha256:") && !update {
        println!("{name}: already pinned to {image} (use --update to re-resolve the tag)");
        return Ok(());
    }
    let tag = image.split('@').next().unwrap();
    let out = Command::new(&ctx.runtime)
        .args([
            "buildx",
            "imagetools",
            "inspect",
            tag,
            "--format",
            "{{json .Manifest}}",
        ])
        .output()
        .context("running `docker buildx imagetools inspect`")?;
    if !out.status.success() {
        bail!(
            "resolving {tag}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let manifest: serde_json::Value = serde_json::from_slice(&out.stdout)?;
    let digest = manifest["digest"]
        .as_str()
        .context("no digest in registry response")?;
    let pinned = format!("{tag}@{digest}");
    if pinned == *image {
        println!("{name}: {tag} still resolves to {digest}");
        return Ok(());
    }
    let path = imp.dir.join(manifest::MANIFEST);
    let text = imp
        .manifest_text
        .replacen(&format!("\"{image}\""), &format!("\"{pinned}\""), 1);
    if text == imp.manifest_text {
        bail!("could not find image = \"{image}\" in {}", path.display());
    }
    std::fs::write(&path, text)?;
    println!("{name}: pinned {pinned}");
    Ok(())
}

fn doctor(ctx: &Ctx) -> Result<bool> {
    let mut ok = true;
    let mut check = |good: bool, what: String| {
        println!("{} {what}", if good { "✅" } else { "❌" });
        ok &= good;
    };
    println!("   repository: {}", ctx.root.display());
    let rt = Command::new(&ctx.runtime)
        .args(["version", "--format", "{{.Server.Version}}"])
        .output();
    match rt {
        Ok(o) if o.status.success() => check(
            true,
            format!(
                "{} server {}",
                ctx.runtime,
                String::from_utf8_lossy(&o.stdout).trim()
            ),
        ),
        _ => check(
            false,
            format!(
                "{} is not installed or its daemon is not running",
                ctx.runtime
            ),
        ),
    }
    match manifest::load_all(&ctx.root) {
        Ok(impls) => {
            let n: usize = impls.iter().map(|i| i.solutions.len()).sum();
            check(
                true,
                format!("{} implementations, {n} solutions", impls.len()),
            );
            for imp in &impls {
                if let Some(image) = &imp.manifest.image
                    && !image.contains("@sha256:")
                {
                    check(
                        false,
                        format!(
                            "{}: image not pinned by digest (cargo aoc toolchain pin {})",
                            imp.name(),
                            imp.name()
                        ),
                    );
                }
            }
            let mut missing = 0;
            for imp in &impls {
                for s in &imp.solutions {
                    if data::input(ctx, imp.year, s.day)?.is_none()
                        || data::answers(ctx, imp.year, s.day)?
                            .expected(s.part)
                            .is_none()
                    {
                        missing += 1;
                    }
                }
            }
            let has_data = ctx.data_dir.is_dir();
            check(
                has_data,
                format!("data directory {}", ctx.data_dir.display()),
            );
            if has_data {
                check(
                    missing == 0,
                    format!("{missing} solution(s) lack an input or answer (cargo aoc fetch YEAR)"),
                );
            }
        }
        Err(e) => check(false, format!("manifests: {e:#}")),
    }
    let session = ctx.var("AOC_SESSION").is_some();
    println!(
        "{} AOC_SESSION {}",
        if session { "✅" } else { "➖" },
        if session {
            "set"
        } else {
            "not set (only needed for `fetch`)"
        }
    );
    if let Some(ca) = &ctx.extra_ca {
        println!("   AOC_EXTRA_CA: {}", ca.display());
    }
    Ok(ok)
}
