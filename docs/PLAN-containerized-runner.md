# Plan: containerized multi-implementation runner, private test data, CI + CLI

> **Status: implemented.** `README.md` is the user guide. This plan is kept for
> the reasoning behind it. Where the implementation differs from the plan:
>
> * **Answers are compared with the last non-empty line of stdout**, not the
>   whole of stdout, so debug prints are tolerated (2025 day 1 prints them).
> * **Rust crates are built with `cargo install --root {cache}/out/...`**,
>   because several crates' package names don't match their directory
>   (e.g. `01/part2` is named `part1`).
> * **The Haskell toolchains install a current `stack` (3.11.1, checksums
>   pinned) next to the original GHC.** The `stack` shipped with the old GHC
>   images can no longer verify Hackage's signing keys. This is the kind of
>   bit-rot the design exists to contain.
> * **Dockerfile-based toolchain images are rebuilt only when the Dockerfile
>   or a file it names changes** (tracked with an image label). Unchanged
>   toolchains work offline and don't touch the registry.
> * **`results.json` is written only with `--save`** (which CI passes), so a
>   local run can't vouch for a solution in CI's place.
> * **The CI workflow has an optional Docker Hub login**, because anonymous
>   pulls are rate-limited. **It also frees disk space**, because the Haskell
>   images are about 6 GB each.
> * **`aoc new` scaffolds from a per-implementation `[new]` section**: a
>   template folder copied with `{{day}}`-style placeholders, then an optional
>   command run in the toolchain container (e.g. `cargo init`).
> * **`cargo aoc` owns the data repository**: it pulls, commits and pushes it
>   itself, and a pre-push hook records answers before solutions are pushed,
>   so you never manage it by hand.
> * **`aoc watch` and `toolchain mirror` were not built.** They remain possible
>   follow-ups.

## 1. How the repository works today

### Layout

```
<year>/index.ts            # TS object: { [language]: Project } — hand-written per year
<year>/<language>/...      # solutions, layout differs per language/year
<year>/data/DD.in          # puzzle input (gitignored: *.in)
<year>/data/DD.P.out       # expected answer (gitignored: *.out)
types.ts / util.ts         # Project type, docker build/run/test helpers
test.ts                    # runs the *current calendar year* only
all.ts                     # runs every year
README.md                  # progress tables, hand-maintained (generate.ts was deleted)
```

A `Project` has `exists(root, day, part)`, an optional `build(day, part)`, a `run(day, part)` returning
`{ args, cwd }`, and a `container` image tag. `util.ts` bind-mounts `<year>/<language>` at `/data`,
runs `docker run <image> <args>` for the build, then again for the run with the input piped to
stdin, and compares stdout to the `.out` file byte-for-byte.

### What's wired up versus what exists

| Implementation | Registered in `index.ts`? | Image | I/O convention |
|---|---|---|---|
| 2019/python | yes | `python:3.12` (floating patch) | stdin ✓ |
| 2019/rust | **no** | – | standalone `.rs` files, reads `opcodes.txt` from cwd |
| 2020/typescript | yes | `denoland/deno:1.46.3` | stdin ✓, imports from `deno.land/x` **at runtime** |
| 2021/haskell, 2022/haskell | **no** (`year = {}`) | – | one stack project, `Main day part`, reads `../inputs/DD.in` |
| 2023/haskell/day7 | **no** | – | separate stack project |
| 2023/typescript | **no** | – | Bun (`for await (const line of console)`) |
| 2023/rust, 2024/rust | yes | `rust:1.82` | stdin ✓, one crate per day/part |
| 2025/rust | yes | `rust:1.82` | stdin ✓ — but **`edition = "2024"` needs Rust ≥ 1.85, so all 14 crates fail to build** |

The 2025 case shows the core problem: the image is chosen per year by copy-paste, so a newer
toolchain can't be used without touching old years, and nothing notices when a solution stops building.

### Defects in the current harness (`util.ts`, `test.ts`, `all.ts`)

1. **The result of the first run after fetching an input is thrown away.** `run()` fetches the input,
   then returns `{ result: false, ... }` without `stdout`. `getProjectResult` destructures `stdout`
   as `undefined`, and `test()` throws, so the run is reported as "Unexpected error". `fetchInput` also
   writes to `<year>/data/`, which may not exist.
2. **stdout is truncated.** `test()` calls `getReader().read()` once, so it only gets the first chunk.
3. **Possible deadlock.** stderr is piped but never read, and a chatty solution fills the pipe buffer.
4. **The build result is ignored.** `build()`'s boolean is discarded, and a failed build still runs.
5. **The run exit code is never checked, and there is no timeout or resource limit.**
6. **Concurrency is unbounded.** Every day and part for every language starts `docker run` at once
   (50+ concurrent `cargo build`s for `all.ts`). The `promises` arrays are built but never awaited.
7. **Exact byte comparison**, so a trailing newline or CRLF difference counts as a failure.
8. **Debug builds.** It runs `cargo build`, then `cargo run`, which re-checks the crate and runs
   unoptimized code. Build output lands in the bind-mounted `target/` and is root-owned on Linux.
9. **Images are referenced by mutable tags**, not digests (`python:3.12` already floats).
10. **There is no README generation.** `generate.ts` was removed, so the README is stale (2023 is
    empty and 2024/2025 are missing).
11. **Toolchain sprawl at the root:** `package.json` with npm deps (some unused, e.g. `markdown-table`),
    `deno.lock`, and a `tsconfig.json` referencing `bun-types`. `.DS_Store` files are committed under `2024/`.
12. **No CI** exists. Per your note, that's because the inputs and answers can't be public.

Inputs were committed briefly in the past (`2020/data/*.in|out` in `7484018`, `2024/data/15.in.1`).
They look like puzzle *example* inputs rather than personal inputs, so the risk is low. Scrubbing them
would need a history rewrite (`git filter-repo`) and a force-push. That's optional and your call.

---

## 2. Target design

### 2.1 Concepts

* **Implementation**: a directory `<year>/<impl-id>/` holding solutions for some subset of days in that
  year, plus an `aoc.toml` manifest. `impl-id` is free-form (`rust`, `rust-simd`, `haskell`, `zig`),
  so a year can have any number of implementations, including several in one language.
* **Toolchain**: a container image **pinned by digest**. Old implementations keep their digest
  forever, and new ones choose a new one. Upgrading a toolchain is a deliberate edit to one manifest.
* **Solution contract** (the only thing every language must obey):
  * The input arrives on **stdin**. The year, day, and part are also passed as args and as env vars
    `AOC_YEAR`, `AOC_DAY`, `AOC_PART`.
  * The answer goes to **stdout**. Leading and trailing whitespace is trimmed before comparison.
  * The process exits 0. stderr is free-form debug output.
  * The run phase has **no network** and has a timeout.
* **Data**: inputs, answers, and examples live **outside this repo** in a private repository
  (see §3). The CLI finds them through `AOC_DATA_DIR`.

### 2.2 Manifest (`<year>/<impl>/aoc.toml`)

```toml
# 2025/rust/aoc.toml
language = "rust"                       # used for README grouping/labels
image    = "docker.io/library/rust:1.85.1-slim@sha256:<index-digest>"
# or: dockerfile = "Dockerfile"         # build a custom toolchain image from this dir
# platform = "linux/amd64"              # only if the image lacks arm64 (old GHCs on Apple Silicon)

# How to discover which (day, part) exist. Placeholders: {day}=7, {dd}=07, {part}=1|2.
# If {part} is absent from the pattern, both parts are assumed to exist.
discover = "{dd}/part{part}/Cargo.toml"

# Commands run inside the container, with this impl's dir mounted at /src (cwd) and a
# persistent per-impl cache volume at /cache. If `build` has no {day}/{part}, it runs once per impl.
build = "cargo build --release --locked --manifest-path {dd}/part{part}/Cargo.toml --target-dir /cache/t/{dd}-{part}"
run   = "/cache/t/{dd}-{part}/release/part{part}"

timeout = "60s"                         # per run; overridable per day below

[env]
CARGO_HOME = "/cache/cargo-home"

[days.12]                               # optional per-day overrides
timeout = "5m"
```

Examples for the trickier existing implementations:

```toml
# 2021/haskell/aoc.toml: one stack project dispatching on argv
language = "haskell"
image    = "ghcr.io/danielpower/aoc-toolchain-haskell:lts-18.18@sha256:..."   # GHC + snapshot pre-baked
discover = "src/Day{day}.hs"
build    = "stack build --allow-different-user"
run      = "stack exec AdventOfCode2021Haskell-exe -- {day} {part}"
[env]
STACK_ROOT = "/opt/stack"   # baked into the image so runs need no network

# 2020/typescript/aoc.toml: remote imports cached during the (networked) build phase
language = "typescript"
image    = "docker.io/denoland/deno:1.46.3@sha256:..."
discover = "{dd}/{part}.ts"
build    = "deno cache {dd}/{part}.ts"
run      = "deno run --cached-only {dd}/{part}.ts"
[env]
DENO_DIR = "/cache/deno"
```

### 2.3 Container execution rules (the "won't break later" guarantees)

| Phase | Network | Mounts | User | Notes |
|---|---|---|---|---|
| build | allowed (deps) | `/src` rw, `/cache` rw (named volume `aoc-<year>-<impl>`) | host uid:gid on Linux | lockfiles enforced (`--locked`, `stack.yaml.lock`, `deno.lock`, `bun.lockb`) |
| run | `--network none` | `/src` ro, `/cache` ro | same | `-i` stdin, timeout, `--memory` cap, `HOME=/cache/home` |

* **Pin by multi-arch index digest**, not a per-platform digest, so the same pin works on amd64 CI
  and an arm64 Mac. Add `platform = "linux/amd64"` only where no arm64 build exists (likely the
  older GHC images).
* **Anything fetched at run time goes into the image or the build phase.** That covers the Haskell
  snapshot and GHC, and Deno remote modules. The run phase never touches the network, so a
  disappearing upstream can't break a run.
* **Mirror long-lived images** (optional, recommended for anything custom): `aoc toolchain mirror`
  pushes a pinned image to `ghcr.io/danielpower/...` so you don't depend on Docker Hub retention.
* Support `docker` and `podman` through `AOC_CONTAINER_RUNTIME`. The flags used are common to both,
  and the existing `:z` suffix suggests podman/SELinux has been used here.

### 2.4 Test data lookup

```
$AOC_DATA_DIR/<year>/<dd>/input.txt
$AOC_DATA_DIR/<year>/<dd>/answers.toml      # part1 = "1234"  part2 = "EHPZPJGL"
$AOC_DATA_DIR/<year>/<dd>/examples/<n>.txt  # optional
$AOC_DATA_DIR/<year>/<dd>/examples.toml     # [[example]] file="1.txt" part1="..." part2="..."
```

* Answers are compared after whitespace trimming. For the "read the ASCII art" puzzles
  (2019 days 8 and 11, 2022 day 10, and so on), `answers.toml` may hold `part2_stdout = """..."""`
  for an exact-render comparison instead of the submitted letters.
* Part counts and day counts come from per-year config. 2015–2024 have 25 days, 2025 has 12, and the
  last day of each year has a single part. Nothing should hardcode `day <= 25` as the current code does.

---

## 3. Keeping inputs and answers private

**Recommendation: a private sibling repo, `DanielPower/AdventOfCode-data`, checked out beside this one.
Don't use a submodule.**

* **Locally:** clone it anywhere and set `AOC_DATA_DIR` (default `./.aoc-data`, gitignored).
* **In CI:** a second `actions/checkout` step with a **read-only deploy key** scoped to that one repo,
  stored as the secret `AOC_DATA_DEPLOY_KEY`. A deploy key is narrower than a PAT and doesn't expire.
* **Populating it:** `aoc fetch <year> [day]` downloads `input.txt` with `AOC_SESSION`. It also
  scrapes your submitted answers from the puzzle page (once a part is solved, the page shows
  "Your puzzle answer was …"), so `answers.toml` mostly fills itself. Follow AoC's automation
  guidance: a User-Agent that identifies the tool, caching (never re-download an existing file),
  and throttling. CI never contacts adventofcode.com. The data repo is the cache of record, which
  also avoids depending on a session cookie that expires.
* A one-off `aoc data import` moves your existing local `<year>/data/DD.in` and `DD.P.out` files into the
  new layout.

Why not the alternatives:
* *git-crypt/sops in the public repo*: this publishes ciphertext of the content permanently, a leaked
  key can never be revoked from history, and key management is more work than a deploy key.
* *GitHub Actions secrets*: these are capped at 48 KB each, so they can't hold years of inputs.
* *A cloud bucket*: this adds infrastructure and credentials for no gain over a private repo.

### Preventing leaks through CI (this matters as much as storage)

The repo is public, so **Actions logs and artifacts are public**. `aoc test --ci` must:

1. **Never print expected answers, actual output, solution stdout, or solution stderr.** It reports
   only `pass`, `wrong-answer`, `build-error`, `runtime-error`, `timeout`, or `no-data`. Build logs
   (compiler errors) are safe to show because they contain no input.
2. Emit `::add-mask::<answer>` for every expected answer before running anything, as defense in depth.
3. Upload no artifacts derived from solution output.
4. **Never use `pull_request_target`, and never run fork PRs with secrets.** On `pull_request`,
   GitHub already withholds secrets from forks. For forks the workflow falls back to **build-only**
   (it compiles changed implementations with no data). Even with `--network none`, an untrusted
   solution could print its input to the log, so untrusted code must never see the data.

---

## 4. CLI (`aoc`)

**Recommendation: Rust, as a standalone crate in `tools/aoc/`, invoked through a cargo alias.**

The current tooling is on Deno, and you're moving off it. Under this plan almost none of the
existing TypeScript survives anyway (§6 Phase 1 replaces all of it), so a language change costs little now.

Why Rust:
* **It doesn't need a separate build step.** A root `.cargo/config.toml` alias makes the
  compile invisible, like running a script:
  ```toml
  [alias]
  aoc = "run -q --release --manifest-path tools/aoc/Cargo.toml --"
  ```
  `cargo aoc test 2025/7` works from anywhere in the repo. The first run compiles in about 30 s,
  and later runs are incremental (sub-second when the tool hasn't changed). You already have
  `cargo` installed for the solutions.
* **The tool can't drift either.** `tools/aoc/rust-toolchain.toml` pins the compiler and
  `Cargo.lock` pins the dependencies. A built binary has no runtime that can be deprecated, the
  same guarantee the plan gives the solutions.
* **It suits the job.** A manifest typed with `serde`, deserialized from TOML, gives validation and
  error messages for free. Subprocess, timeout and thread-pool handling is straightforward. A static
  binary is easy to hand to CI.
* **CI builds it.** CI runs `cargo build --release` with `Swatinem/rust-cache` (about 10–20 s when
  cached). Publishing release binaries is optional (§6 Phase 4) and only helps on machines
  without Rust.

Layout and crates (kept deliberately small):
```
tools/aoc/
  Cargo.toml  Cargo.lock  rust-toolchain.toml
  src/main.rs       # clap (derive) subcommands
  src/manifest.rs   # aoc.toml: serde + toml, validation, placeholder expansion
  src/discover.rs   # "{dd}/part{part}/Cargo.toml" → regex → walkdir → (day, part) set
  src/runtime.rs    # docker/podman invocation, build/run, timeouts (wait-timeout), kill
  src/data.rs       # AOC_DATA_DIR, answers.toml, fetch + answer scraping (ureq, regex)
  src/results.rs    # results.json (serde_json), cache keys (sha2)
  src/readme.rs     # marker-block rendering
.cargo/config.toml  # the alias above
```
Concurrency uses `std::thread::scope` with a channel-fed worker pool (or `rayon` with a sized
pool), so no async runtime is needed.

**Workspace gotcha:** don't put a `[workspace]` `Cargo.toml` at the repo root. Cargo searches
upward for one, and the solution crates (`2024/rust/01/part1`, ...) would then fail with
"believes it's in a workspace when it's not". The tool lives in its own package under `tools/aoc/`,
reached through `--manifest-path`.

**Timeout gotcha (this applies in any language):** killing the `docker run` client process does
*not* stop the container. Start each run with `--name aoc-<random>` and `--init`, and on timeout run
`docker kill <name>` and then reap the client.

Runner-up: **Node 24 LTS with native TypeScript type-stripping** (`node tools/aoc/main.ts`, no
build step). It needs only erasable syntax (no enums or namespaces) and a `.ts` import extension,
which this repo already uses. Choose it if a compile-free script matters more than the drift
guarantee. It still needs `npm ci` for a TOML parser, and it re-introduces a runtime to keep current.

**Deno-based solutions are unaffected.** `2020/typescript` (Deno 1.46.3) and `2023/typescript`
(Bun) run inside digest-pinned images, so a deprecated runtime doesn't break them. Mirror those two
images to GHCR (`aoc toolchain mirror`) so they don't depend on Docker Hub keeping them.

```
aoc test   [SEL] [--impl ID] [--examples] [--jobs N] [--ci] [--json FILE] [--changed]
aoc run    YEAR DAY PART [--impl ID] [--input FILE|-] [--native]   # print the answer
aoc build  [SEL] [--impl ID]
aoc fetch  YEAR [DAY]                         # input + answers → $AOC_DATA_DIR
aoc new    YEAR DAY --impl ID [--lang rust]   # scaffold from templates/<lang>/
aoc readme [--check]                          # regenerate README block from results.json
aoc toolchain pin  <impl>                     # resolve tag → digest, write into aoc.toml
aoc toolchain mirror <impl>                   # copy image to GHCR
aoc doctor                                    # runtime, data dir, session cookie, images present
SEL = 2025 | 2025/7 | 2025/7/2   (default: everything)
```

Behaviour:
* A bounded worker pool (`--jobs`, default = CPU count). Builds are de-duplicated per impl, or per
  day and part, and each build finishes before its runs start.
* Output is a live table locally. `--json` emits machine-readable results, `--ci` adds the
  redaction rules from §3, and the exit code is non-zero on any failure.
* `--native` skips the container and runs the manifest commands on the host. This is for fast
  iteration on the current day only, with no reproducibility guarantee, and it is never used in CI.
* `--examples` runs against `examples.toml`, which is useful before your real input is unlocked.
* Optional extra: `aoc watch YEAR DAY` re-runs examples and the real input on file change.

### Results cache (`results.json`, committed)

```json
{
  "2025/rust/07/1": {
    "status": "pass",
    "ms": 14,
    "key": "sha256(impl files + aoc.toml + image digest + data files for that day)",
    "checked": "2026-11-12T18:00:00Z"
  }
}
```

It contains no answers or input. It is the source for README generation and lets CI skip work whose
`key` is unchanged. Durations are measured inside the container by a tiny wrapper, so they exclude
container start-up, and they can be shown in the README as an optional column.

---

## 5. CI (`.github/workflows/verify.yml`)

```yaml
on:
  push:          { branches: [main] }
  pull_request:  {}
  schedule:      [{ cron: "23 7 * * 1" }]   # weekly full re-verify: catches image/registry rot
  workflow_dispatch: {}

concurrency: { group: verify-${{ github.ref }}, cancel-in-progress: true }

jobs:
  verify:
    runs-on: ubuntu-latest
    permissions: { contents: write }
    env:
      TRUSTED: ${{ github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name == github.repository }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/checkout@v4
        if: env.TRUSTED == 'true'
        with:
          repository: DanielPower/AdventOfCode-data
          ssh-key: ${{ secrets.AOC_DATA_DEPLOY_KEY }}
          path: .aoc-data
      - uses: Swatinem/rust-cache@v2               # caches the aoc tool build
        with: { workspaces: tools/aoc }
      - run: cargo build --release --manifest-path tools/aoc/Cargo.toml   # rustup on the runner honours rust-toolchain.toml
      - run: echo "$PWD/tools/aoc/target/release" >> "$GITHUB_PATH"
      - uses: actions/cache@v4                     # cargo/stack/deno build caches for the solutions
        with: { path: ~/.aoc-cache, key: aoc-${{ hashFiles('**/aoc.toml', '**/*.lock') }} }
      - name: Test
        run: |
          if [ "$TRUSTED" = true ]; then
            aoc test --ci --changed --json results.json   # --changed: skip unchanged keys
          else
            aoc build --changed                          # forks: compile only
          fi
      - name: Update README
        if: github.event_name != 'pull_request' && github.ref == 'refs/heads/main'
        run: |
          aoc readme
          git diff --quiet README.md results.json || {
            git config user.name  "github-actions[bot]"
            git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
            git commit -am "Update progress [skip ci]" && git push
          }
```

* Pushes made with `GITHUB_TOKEN` don't re-trigger workflows, so there is no loop. If you later add
  branch protection on `main`, allow the Actions bot to bypass it, or have CI open a PR instead.
* The scheduled run uses `--all` (it ignores the cache), which proves old solutions still build
  from their pinned images.
* If it gets slow, the next step is to fan out: a `plan` job lists the stale impls, then a matrix job
  runs one impl per runner, then an aggregate job merges the JSON and commits the README.

### README format

The CLI rewrites only the region between `<!-- aoc:progress:start -->` and `<!-- aoc:progress:end -->`,
so hand-written prose around it survives. Each year gets a table with one row per implementation:

```
## 2025 — 14/23 ⭐
| Impl | ⭐ | 1 2 3 4 5 6 7 8 9 10 11 12 |
|------|----|----------------------------|
| rust | 14 | ★★★★★★★☆☆☆☆☆ ...           |
```

Each day's cell shows both parts passing (★★ or ✅), part 1 only (★ or 🟡), failing (❌), or not
attempted (blank). Not attempted is now distinct from failing; the current README uses ❌ for both.

---

## 6. Migration, in order

**Phase 0: data (do this first; it unblocks everything)**
1. Create a private `AdventOfCode-data` repo and run `aoc data import` on your local `*/data/` files.
2. Run `aoc fetch` for each year with solutions so missing inputs and answers get filled.
3. Add the deploy key and the `AOC_DATA_DEPLOY_KEY` secret.

**Phase 1: CLI core** (replaces `test.ts`, `all.ts`, `util.ts`, `types.ts`, and every `<year>/index.ts`)
1. Manifest parsing and validation, discovery, and selector parsing.
2. The container runner with the §2.3 rules, a bounded pool, full stdout/stderr draining,
   exit-code and timeout handling, and trimmed comparison. This fixes defects 1–8 by construction.
3. `test`, `run`, `build`, `--json`, `--ci` redaction, and `results.json` plus cache keys.
4. `readme` (with `--check`), and `toolchain pin`.
5. Tooling cleanup: add `tools/aoc/` and the `.cargo/config.toml` alias, and add `tools/aoc/target/`
   to `.gitignore`. Delete the Deno/Node tooling at the root: `package.json`, `deno.lock`,
   `tsconfig.json` (`bun-types`), `types.ts`, `util.ts`, `test.ts`, `all.ts`, and every
   `<year>/index.ts`. Also delete the committed `.DS_Store` files. Keep the old tooling until the
   Rust CLI passes the same solutions, then remove it in one commit.

**Phase 2: move each implementation onto the contract** (one commit each, verified with `aoc test`)

| Impl | Toolchain to pin | Code changes |
|---|---|---|
| 2019/python | `python:3.12.x` digest | none |
| 2019/rust | any current `rust` digest | read stdin instead of `opcodes.txt`; build with `rustc -O {dd}/{part}.rs -o /cache/{dd}-{part}` |
| 2020/typescript | `denoland/deno:1.46.3` digest | none; `deno cache` in the build step |
| 2021/haskell | custom image: lts-18.18 (GHC 8.10.7) pre-baked; probably `linux/amd64` only | `Main.hs`: read input with `getContents` instead of `readFile ("../inputs/" ++ file)` |
| 2022/haskell | custom image: lts-21.21 (GHC 9.4.7) | same `Main.hs` change |
| 2023/haskell | custom image: lts-22.0 (GHC 9.6.3) | discovery `day{day}/`; build and run per day |
| 2023/typescript | `oven/bun:1.0.x` digest (match `bun.lockb`) | none |
| 2023/rust, 2024/rust | `rust:1.82.0` digest (what they're known to work with) | none (release builds) |
| 2025/rust | `rust:1.85+` digest | none; **this fixes the edition-2024 build failure** |

Pinning 2023 and 2024 to 1.82 and 2025 to a newer version shows the point of the design: each
implementation carries its own toolchain.

**Phase 3: CI**
1. Add `verify.yml`, run it with `workflow_dispatch`, and check the logs are redacted (deliberately break one answer to confirm).
2. Seed the README markers and let the first green run on `main` regenerate it.

**Phase 4: optional extras**
* `aoc new` templates per language (Cargo crate, `stack` project, and so on) that already obey the contract.
* `aoc watch`, timing column in README, `aoc toolchain mirror` for the custom Haskell images.
* An `aoc submit` command (it needs careful rate limiting).
* A release workflow (on tag) that publishes `aoc` binaries for `x86_64-unknown-linux-gnu` and
  `aarch64-apple-darwin`, for use on machines without Rust. Not needed for CI or your own machine.

---

## 7. Decisions for you

1. **CLI language:** Rust run through the `cargo aoc` alias (recommended), or Node 24 with native
   type-stripping (`node tools/aoc/main.ts`, no build step) if you'd rather stay in TypeScript.
   Deno is out either way.
2. **Data storage:** private repo with a deploy key (recommended) or git-crypt in this repo.
3. **History scrub** of the old `2020/data` and `2024/data` example files: optional, needs a force-push.
4. **Results cache committed** (`results.json`, recommended, makes CI incremental and the README
   reproducible locally) or kept only in the Actions cache.
