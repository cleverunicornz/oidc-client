# D-000016 — MSRV baseline is compilation-proven, not inherited metadata

## Status

accepted

## Date

2026-09-23

## Context

The manifest carried `rust-version = "1.65"` verbatim from the upstream
import (D-000004 item 2 keeps every non-metadata field verbatim, and
`rust-version` was not a metadata delta). The claim is not compilable
today: a fresh resolution of this crate's manifest selects dependencies
whose own declared minimum rust versions exceed 1.65. The same review
that flagged the overclaim also flagged `Cargo-1.65.lock`, retained as an
"inert reference lock" (D-000002) whose rejection of removal in
D-000004 ("loses the MSRV reference lock the upstream shipped for its
1.65.0 gate") presented it as an MSRV strategy. Neither the 1.65 claim
nor the lock survives contact with the toolchains that actually exist.

## Evidence

- Fresh offline resolution at worktree head 1d47ff9 (no `Cargo.lock`
  present; resolution from the local crates.io index cache) selected
  `serde_with 3.23.0`, `ed25519-dalek 2.2.0`, and `base64ct 1.8.3`. Each
  crate's own published manifest declares `rust-version = "1.88"`,
  `"1.81"`, and `"1.85"` respectively (read from
  `~/.cargo/registry/src/index.crates.io-*/<crate>/Cargo.toml`). The
  resolved graph's floor therefore exceeds 1.65 and no fresh resolution
  can compile under 1.65's gate.
- `Cargo-1.65.lock`'s root package entry is `openidconnect 4.0.1`
  (verified verbatim); it does not name `oidc-client` at all. A probe
  copying it to `Cargo.lock` alongside this manifest and running
  `cargo +1.96.0 check --offline --locked` fails with the verbatim
  error: `error: cannot update the lock file /tmp/lockprobe/Cargo.lock
  because --locked was passed to prevent this` — the lock cannot back a
  `--locked` build of this crate.
- Locally installed rustup toolchains: `1.96.0`,
  `1.98.0` (active, default), `nightly` (1.100.0-nightly,
  commit-date 2026-09-19). Installing additional toolchains locally is
  prohibited by fleet policy; missing capabilities are requested, not
  installed.
- Compilation proof at commit f80568881fccd9e32ad99f593b39b6e54f9fa59a
  (situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md):
  the battery was re-executed at this reachable head after the
  originally cited stream-worktree head
  e0860535f99aa50ad3d45befcea7ed1644864cc0 proved unresolvable on the
  branch (situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md);
  fresh-resolution `cargo check --offline` and
  `cargo check --offline --all-features` exit 0 on 1.96.0 and on 1.98.0;
  a forced rustc re-invocation on the crate confirms 1.96.0 compiles the
  final manifest; the nightly run (informational) also passes.
- `.github/workflows/ci.yml` has a single `ci` job (capabilities,
  formatting, clippy, tests, dependency policy) on the pinned gate
  toolchain; no MSRV-pinned job exists.
- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the import-fidelity regime and the rejected alternative this
  decision supersedes on the touched surface.
- situation/decisions/D-000002-version-oidc-client-4-1-0-continuing-upstream-lineage.md
  — the lock's "inert artifact" retention rationale, also superseded for
  this surface by the removal below.

## Decision

1. `Cargo.toml` declares `rust-version = "1.96"`: the oldest toolchain
   proven by recorded compilation evidence to build the crate's default
   feature set and `--all-features` from a fresh resolution. Supported
   minimums are never inferred from manifest metadata alone.
2. `Cargo-1.65.lock` is removed. This supersedes, for this surface, the
   D-000004 rejected alternative "Dropping `Cargo-1.65.lock`: loses the
   MSRV reference lock": the lock's root package is not this crate, it
   cannot be used with `--locked`, cargo does not read that filename,
   and presenting it as an MSRV strategy is misleading. D-000004 and
   D-000002 remain unedited; supersession is append-only through this
   record.
3. README.md states the MSRV position in its authored voice (upstream
   `README.upstream.md` is untouched and keeps its own MSRV prose).
4. The claim is carried by the direct Promise
   situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md
   under situation/oracles/O-000016-judge-fresh-resolution-compiles-at-declared-floor.md,
   witnessed by
   situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md.
5. These changes are deliberate maintenance deltas of the continued
   crate — manifest metadata, authored orientation, and repository
   hygiene — authorized by this stream's mandate; they supersede the
   D-000004 import-fidelity regime for exactly the touched surface
   (`Cargo.toml`'s `rust-version` field, `Cargo-1.65.lock`, README.md).
   A future upstream re-sync classifies them under this decision, not
   the import-fidelity table.

## Why

An MSRV claim is a contract with consumers and with every future
resolver run; a floor the dependency graph already exceeds is false the
day it ships. The honest floor is the oldest toolchain that demonstrably
compiles, and with only 1.96.0 and 1.98.0 available locally, the proven
floor is 1.96.0. The reference lock compounded the problem: it implies a
reproducible MSRV gate that cannot exist (wrong root package, unusable
with `--locked`), so keeping it preserves a misleading artifact rather
than provenance. Provenance for the upstream 1.65 gate lives in the
situation records and `README.upstream.md`, which stay byte-identical.

## Rejected alternatives

- Keep `rust-version = "1.65"`: rejected — the freshly resolved graph
  (serde_with 3.23.0 at 1.88, ed25519-dalek 2.2.0 at 1.81, base64ct
  1.8.3 at 1.85) cannot compile under it; the claim is false as
  published.
- Declare a lower inferred floor (1.81, 1.85, or 1.88): rejected — no
  toolchain in that range is installed locally, installing one is
  prohibited, and metadata-derived minimums are exactly the inference
  this decision forbids; the floor stays unproven (G-000026).
- Keep `Cargo-1.65.lock` as an MSRV reference: rejected — root package
  mismatch (`openidconnect 4.0.1`), `--locked` fails against this
  manifest, and its 1.65-era resolution no longer describes any
  buildable state of this crate.

## Consequences

`cargo +1.96.0 check` (default and `--all-features`, fresh resolution)
passes at the declaring commit; 1.98.0 likewise; the nightly run passes
informationally. No replacement `Cargo.lock` is committed; consumers and
CI resolve fresh. The RUSTSEC-2025-0056 rationale in `deny.toml` claims
"the resolver holds backtrace at 0.3.71 under rust-version 1.65"; the
fresh resolution after this change still selects backtrace 0.3.71 →
miniz_oxide 0.7.4 → adler 1.0.2, so the advisory path it disposes of
remains present and that record stays factually accurate. The true
lower dependency-graph floor (unproven below the compilation-verified
1.96) and the absence of an MSRV-pinned CI job are recorded in
situation/gaps/G-000026-true-msrv-floor-unproven-and-no-msrv-ci-job.md.

## Revisit when

A toolchain older than 1.96 becomes locally available (via a fleet
capability request) and can probe the true floor; or the gate toolchain
moves, in which case the declared floor is re-proven with a new
witness under P-000015/O-000016.

## Provenance

Recorded 2026-09-23 by Stream E of the pre-publication hardening branch
`fix/prepublication-hardening` (worktree head
e0860535f99aa50ad3d45befcea7ed1644864cc0 carries the manifest, README,
and lock-removal deltas; the compilation proof battery was run against
that commit the same day).
