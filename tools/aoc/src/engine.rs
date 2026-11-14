//! Executing build/run commands, either inside the implementation's pinned
//! toolchain container or (with `--native`) directly on the host.

use crate::ctx::Ctx;
use crate::manifest::Impl;
use crate::model::Solution;
use anyhow::{Context, Result, bail};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

/// Label put on every container we start, so stale ones can be cleaned up.
const LABEL: &str = "aoc.runner=1";
/// Where the extra CA bundle is mounted during builds.
const CA_PATH: &str = "/etc/ssl/certs/ca-certificates.crt";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Network allowed; source and cache writable.
    Build,
    /// No network; source and cache read-only.
    Run,
}

pub struct Exec<'a> {
    pub imp: &'a Impl,
    /// Toolchain image (ignored in native mode).
    pub image: &'a str,
    pub sol: Option<Solution>,
    pub template: &'a str,
    pub phase: Phase,
    pub stdin: Option<&'a [u8]>,
    pub timeout: Duration,
}

#[derive(Debug)]
pub enum Exit {
    Code(i32),
    Signal,
    TimedOut,
}

pub struct Outcome {
    pub exit: Exit,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// Time the command itself ran (container start-up excluded where the
    /// runtime reports it).
    pub elapsed: Duration,
}

impl Outcome {
    pub fn ok(&self) -> bool {
        matches!(self.exit, Exit::Code(0))
    }

    pub fn stderr_tail(&self, n: usize) -> String {
        let s = String::from_utf8_lossy(&self.stderr);
        let lines: Vec<&str> = s.lines().collect();
        lines[lines.len().saturating_sub(n)..].join("\n")
    }

    pub fn describe(&self) -> String {
        match self.exit {
            Exit::Code(0) => "ok".into(),
            Exit::Code(c) => format!("exit code {c}"),
            Exit::Signal => "killed by a signal".into(),
            Exit::TimedOut => "timed out".into(),
        }
    }
}

pub struct Engine<'a> {
    pub ctx: &'a Ctx,
    pub native: bool,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Engine<'_> {
    /// Host directory mounted at /cache for an implementation.
    pub fn cache_dir(&self, imp: &Impl) -> Result<PathBuf> {
        let dir = self.ctx.cache_dir.join(format!("{}-{}", imp.year, imp.id));
        std::fs::create_dir_all(dir.join("home"))
            .with_context(|| format!("creating {}", dir.display()))?;
        Ok(dir)
    }

    pub fn exec(&self, job: &Exec) -> Result<Outcome> {
        if self.native {
            self.exec_native(job)
        } else {
            self.exec_container(job)
        }
    }

    fn env(&self, job: &Exec, cache: &str, src: &str) -> Vec<(String, String)> {
        let mut env = vec![
            ("HOME".to_string(), format!("{cache}/home")),
            ("AOC_YEAR".to_string(), job.imp.year.to_string()),
        ];
        if let Some(sol) = job.sol {
            env.push(("AOC_DAY".into(), sol.day.to_string()));
            env.push(("AOC_PART".into(), sol.part.to_string()));
        }
        for (k, v) in &job.imp.manifest.env {
            env.push((k.clone(), job.imp.expand(v, job.sol, cache, src)));
        }
        env
    }

    fn exec_native(&self, job: &Exec) -> Result<Outcome> {
        let cache = self.cache_dir(job.imp)?;
        let (cache, src) = (
            cache.to_string_lossy().into_owned(),
            job.imp.dir.to_string_lossy().into_owned(),
        );
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(job.imp.expand(job.template, job.sol, &cache, &src))
            .current_dir(&job.imp.dir)
            .envs(self.env(job, &cache, &src));
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
        let start = Instant::now();
        let (exit, stdout, stderr) = drive(cmd, job.stdin, job.timeout, |child| {
            // Kill the whole process group, not just `sh`.
            let _ = Command::new("kill")
                .args(["-KILL", "--", &format!("-{}", child.id())])
                .status();
            let _ = child.kill();
        })?;
        Ok(Outcome {
            exit,
            stdout,
            stderr,
            elapsed: start.elapsed(),
        })
    }

    fn exec_container(&self, job: &Exec) -> Result<Outcome> {
        let imp = job.imp;
        let cache = self.cache_dir(imp)?;
        let name = format!(
            "aoc-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let run = job.phase == Phase::Run;
        let mode = if run { ":ro,z" } else { ":z" };

        let mut args: Vec<String> =
            vec!["run".into(), "--name".into(), name.clone(), "--init".into()];
        args.extend(["--label".into(), LABEL.into()]);
        if job.stdin.is_some() {
            args.push("-i".into());
        }
        if run {
            args.extend(["--network".into(), "none".into()]);
        }
        if let Some(p) = &imp.manifest.platform {
            args.extend(["--platform".into(), p.clone()]);
        }
        args.extend(["-v".into(), format!("{}:/src{mode}", imp.dir.display())]);
        args.extend(["-v".into(), format!("{}:/cache{mode}", cache.display())]);
        args.extend(["-w".into(), "/src".into()]);
        if let Some(user) = host_user(&self.ctx.root) {
            args.extend(["--user".into(), user]);
        }
        let mut env = self.env(job, "/cache", "/src");
        if let (Phase::Build, Some(ca)) = (job.phase, &self.ctx.extra_ca) {
            args.extend(["-v".into(), format!("{}:{CA_PATH}:ro", ca.display())]);
            for var in [
                "SSL_CERT_FILE",
                "DENO_CERT",
                "NODE_EXTRA_CA_CERTS",
                "CARGO_HTTP_CAINFO",
                "REQUESTS_CA_BUNDLE",
                "PIP_CERT",
            ] {
                env.push((var.into(), CA_PATH.into()));
            }
        }
        for (k, v) in env {
            args.extend(["-e".into(), format!("{k}={v}")]);
        }
        // Bypass image entrypoints (some wrap commands or run their own init):
        // every command runs as `sh -c <command>`.
        args.extend(["--entrypoint".into(), "sh".into(), job.image.to_string()]);
        args.extend([
            "-c".into(),
            imp.expand(job.template, job.sol, "/cache", "/src"),
        ]);

        let rt = &self.ctx.runtime;
        let _cleanup = RemoveOnDrop {
            runtime: rt,
            name: &name,
        };
        let mut cmd = Command::new(rt);
        cmd.args(&args);
        let start = Instant::now();
        let (exit, stdout, stderr) = drive(cmd, job.stdin, job.timeout, |child| {
            // Killing the client does not stop the container; kill it by name.
            let _ = Command::new(rt)
                .args(["kill", &name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            let _ = child.kill();
        })?;
        let wall = start.elapsed();
        if let Exit::Code(125) = exit {
            bail!(
                "{rt} could not start the container: {}",
                String::from_utf8_lossy(&stderr).trim()
            );
        }
        let elapsed = container_runtime(rt, &name).unwrap_or(wall);
        Ok(Outcome {
            exit,
            stdout,
            stderr,
            elapsed,
        })
    }

    /// Make sure the implementation's toolchain image is available; returns
    /// the image reference to run.
    pub fn prepare_image(&self, imp: &Impl) -> Result<String> {
        if self.native {
            return Ok(String::new());
        }
        let rt = &self.ctx.runtime;
        if let Some(image) = &imp.manifest.image {
            if !image.contains("@sha256:") {
                eprintln!(
                    "warning: {}: image {image:?} is not pinned by digest (run `aoc toolchain pin {}`)",
                    imp.name(),
                    imp.name()
                );
            }
            let present = Command::new(rt)
                .args(["image", "inspect", image])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?
                .success();
            if !present {
                eprintln!("pulling {image}");
                let mut cmd = Command::new(rt);
                cmd.arg("pull");
                if let Some(p) = &imp.manifest.platform {
                    cmd.args(["--platform", p]);
                }
                let out = cmd
                    .arg(image)
                    .output()
                    .with_context(|| format!("running {rt} pull"))?;
                if !out.status.success() {
                    bail!(
                        "pulling {image} failed:\n{}",
                        String::from_utf8_lossy(&out.stderr).trim()
                    );
                }
            }
            return Ok(image.clone());
        }
        let dockerfile = imp
            .manifest
            .dockerfile
            .as_ref()
            .expect("validated in manifest");
        let tag = format!("aoc-toolchain/{}-{}", imp.year, imp.id.to_lowercase());
        // Rebuild only when the Dockerfile or a file it names changed, so
        // unchanged toolchains work offline and don't touch the registry.
        let key = dockerfile_key(&imp.dir, dockerfile)?;
        let current = Command::new(rt)
            .args([
                "image",
                "inspect",
                "--format",
                "{{index .Config.Labels \"aoc.toolchain-key\"}}",
                &tag,
            ])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
        if current.as_deref() == Some(key.as_str()) {
            return Ok(tag);
        }
        eprintln!(
            "building toolchain image {tag} from {} (slow the first time)",
            self.ctx.rel(&imp.dir.join(dockerfile)).display()
        );
        let mut cmd = Command::new(rt);
        cmd.arg("build")
            .arg("-f")
            .arg(imp.dir.join(dockerfile))
            .args(["-t", &tag]);
        cmd.arg("--label").arg(format!("aoc.toolchain-key={key}"));
        if let Some(p) = &imp.manifest.platform {
            cmd.args(["--platform", p]);
        }
        if let Some(ca) = &self.ctx.extra_ca {
            cmd.arg("--secret")
                .arg(format!("id=extra-ca,src={}", ca.display()));
        }
        let out = cmd
            .arg(&imp.dir)
            .output()
            .with_context(|| format!("running {rt} build"))?;
        if !out.status.success() {
            let log = String::from_utf8_lossy(&out.stderr);
            let tail: Vec<&str> = log.lines().rev().take(40).collect();
            bail!(
                "building {tag} failed:\n{}",
                tail.into_iter().rev().collect::<Vec<_>>().join("\n")
            );
        }
        Ok(tag)
    }

    /// Remove stopped containers left behind by an interrupted run.
    pub fn cleanup_stale(&self) {
        if !self.native {
            let _ = Command::new(&self.ctx.runtime)
                .args([
                    "container",
                    "prune",
                    "-f",
                    "--filter",
                    &format!("label={LABEL}"),
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}

struct RemoveOnDrop<'a> {
    runtime: &'a str,
    name: &'a str,
}

impl Drop for RemoveOnDrop<'_> {
    fn drop(&mut self) {
        let _ = Command::new(self.runtime)
            .args(["rm", "-f", self.name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

/// On Linux, run containers as the owner of the checkout so files written to
/// bind mounts (build output, caches) are not owned by root. Docker Desktop on
/// macOS maps ownership itself.
fn host_user(root: &Path) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::metadata(root).ok()?;
        if meta.uid() != 0 {
            return Some(format!("{}:{}", meta.uid(), meta.gid()));
        }
    }
    let _ = root;
    None
}

/// Hash of a Dockerfile plus every file in its context that it mentions by
/// path (e.g. `COPY stack.yaml stack.yaml.lock ./`).
fn dockerfile_key(dir: &Path, dockerfile: &str) -> Result<String> {
    use sha2::{Digest, Sha256};
    let text = std::fs::read_to_string(dir.join(dockerfile))?;
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    let mut files: Vec<PathBuf> = ignore::WalkBuilder::new(dir)
        .require_git(false)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_some_and(|t| t.is_file()))
        .map(|e| e.into_path())
        .filter(|p| {
            p.strip_prefix(dir)
                .map(|rel| text.contains(&*rel.to_string_lossy().replace('\\', "/")))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    for f in files {
        h.update(
            f.strip_prefix(dir)
                .unwrap_or(&f)
                .to_string_lossy()
                .as_bytes(),
        );
        h.update(std::fs::read(&f)?);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// How long the container's process ran, from the runtime's own timestamps.
fn container_runtime(rt: &str, name: &str) -> Option<Duration> {
    let out = Command::new(rt)
        .args([
            "inspect",
            "--format",
            "{{.State.StartedAt}} {{.State.FinishedAt}}",
            name,
        ])
        .output()
        .ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    let mut it = text.split_whitespace();
    let started = humantime::parse_rfc3339_weak(it.next()?).ok()?;
    let finished = humantime::parse_rfc3339_weak(it.next()?).ok()?;
    finished.duration_since(started).ok()
}

/// Spawn `cmd`, feed stdin, drain stdout/stderr concurrently (so a chatty
/// process can't deadlock on a full pipe), and enforce the timeout.
fn drive(
    mut cmd: Command,
    stdin: Option<&[u8]>,
    timeout: Duration,
    on_timeout: impl FnOnce(&mut Child),
) -> Result<(Exit, Vec<u8>, Vec<u8>)> {
    cmd.stdin(if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .with_context(|| format!("starting {:?}", cmd.get_program()))?;
    let mut out_pipe = child.stdout.take().unwrap();
    let mut err_pipe = child.stderr.take().unwrap();
    let in_pipe = child.stdin.take();
    std::thread::scope(|s| {
        if let (Some(mut pipe), Some(data)) = (in_pipe, stdin) {
            s.spawn(move || {
                // The process may exit without reading everything; that's fine.
                let _ = pipe.write_all(data);
            });
        }
        let out = s.spawn(move || {
            let mut buf = Vec::new();
            let _ = out_pipe.read_to_end(&mut buf);
            buf
        });
        let err = s.spawn(move || {
            let mut buf = Vec::new();
            let _ = err_pipe.read_to_end(&mut buf);
            buf
        });
        let exit = match child.wait_timeout(timeout)? {
            Some(status) => exit_of(status),
            None => {
                on_timeout(&mut child);
                child.wait()?;
                Exit::TimedOut
            }
        };
        Ok((exit, out.join().unwrap(), err.join().unwrap()))
    })
}

fn exit_of(status: ExitStatus) -> Exit {
    match status.code() {
        Some(c) => Exit::Code(c),
        None => Exit::Signal,
    }
}
