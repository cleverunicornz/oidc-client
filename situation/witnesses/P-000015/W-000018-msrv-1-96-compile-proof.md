# W-000018 — Fresh-resolution compile proof on 1.96.0 and 1.98.0

## Promise

situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md

## Oracle

situation/oracles/O-000016-judge-fresh-resolution-compiles-at-declared-floor.md

## Result

PASS — every verified-set toolchain (1.96.0, 1.98.0) exited 0 on
fresh-resolution `cargo check` (default and `--all-features`) at the
head below, forced rustc re-invocation included, and the declared floor
equals the oldest installed stable toolchain.

## Head

f80568881fccd9e32ad99f593b39b6e54f9fa59a — the reachable branch head on
`fix/prepublication-hardening` that this proof anchors to. The original
2026-09-23 battery ran at stream-worktree head
e0860535f99aa50ad3d45befcea7ed1644864cc0, which does not resolve on the
branch (situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md);
that observation is retained as history in this witness.

## Observed

2026-09-24 (re-anchoring battery; the original observation of
2026-09-23 is retained under `Observation history` below).

## Evidence

- `situation/witnesses/evidence/W-000018/msrv-check-1.96.0-at-f805688.log`
  retains the full verbatim 1.96.0 battery at the head above:
  `rustc +1.96.0 -vV` (release 1.96.0), then
  `rm -f Cargo.lock && cargo +1.96.0 check --offline` and
  `rm -f Cargo.lock && cargo +1.96.0 check --offline --all-features`,
  each `Finished \`dev\` profile` with `exit=0`. Resolution was fresh —
  each command begins by removing any `Cargo.lock`.
- `situation/witnesses/evidence/W-000018/msrv-check-1.96.0-recheck-at-f805688.log`
  retains `touch src/lib.rs && cargo +1.96.0 check --offline
  --all-features` at the same head — a forced fresh rustc invocation on
  the crate (`Checking oidc-client v4.1.0` … `Finished`, `exit=0`),
  corroborating that 1.96.0 compiles the final manifest itself, not only
  the cached dependency artifacts.
- `situation/witnesses/evidence/W-000018/msrv-check-1.98.0-at-f805688.log`
  retains the 1.98.0 battery: `rustc +1.98.0 -vV` (release 1.98.0),
  then both fresh-resolution commands ending `Finished \`dev\` profile`
  with `exit=0`, with the crate re-checked
  (`Checking oidc-client v4.1.0`).
- `situation/witnesses/evidence/W-000018/msrv-check-nightly-informational-at-f805688.log`
  retains the informational nightly run (1.100.0-nightly,
  `--all-features`, `exit=0`); nightly is outside the verified set.
- `situation/witnesses/evidence/W-000018/msrv-toolchain-inventory-at-f805688.log`
  retains `rustup toolchain list` at proof time:
  nightly-x86_64-unknown-linux-gnu,
  1.96.0-x86_64-unknown-linux-gnu,
  1.98.0-x86_64-unknown-linux-gnu (active, default) — `exit=0`.
- Retained 2026-09-23 history battery at stream-worktree head
  e0860535f99aa50ad3d45befcea7ed1644864cc0 (unresolvable on the branch,
  G-000029; see `Observation history` below):
  `situation/witnesses/evidence/W-000018/msrv-check-1.96.0.log`,
  `situation/witnesses/evidence/W-000018/msrv-check-1.96.0-recheck.log`,
  `situation/witnesses/evidence/W-000018/msrv-check-1.98.0.log`,
  `situation/witnesses/evidence/W-000018/msrv-check-nightly-informational.log`,
  and `situation/witnesses/evidence/W-000018/msrv-toolchain-inventory.log`.
- `situation/witnesses/evidence/W-000018/SHA256SUMS` records SHA-256
  digests for all ten retained files, grouped by head.

## Observation history

The 2026-09-23 PASS at e0860535f99aa50ad3d45befcea7ed1644864cc0 was a
stream-worktree observation; that commit does not resolve on branch
`fix/prepublication-hardening`
(situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md).
Its evidence logs (`msrv-check-1.96.0.log`,
`msrv-check-1.96.0-recheck.log`, `msrv-check-1.98.0.log`,
`msrv-check-nightly-informational.log`,
`msrv-toolchain-inventory.log` under
`situation/witnesses/evidence/W-000018/`) remain retained and digested
in `SHA256SUMS`. The observation itself is unchanged; its provenance
was re-anchored on 2026-09-24 by re-executing the full battery at
reachable head f80568881fccd9e32ad99f593b39b6e54f9fa59a, and the PASS
above rests on that re-execution.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — 1.96.0 and 1.98.0 both exit 0 on both fresh-resolution commands at f80568881fccd9e32ad99f593b39b6e54f9fa59a (situation/witnesses/evidence/W-000018/msrv-check-1.96.0-at-f805688.log, situation/witnesses/evidence/W-000018/msrv-check-1.98.0-at-f805688.log); the forced 1.96.0 recheck confirms a real rustc invocation on the crate at the final manifest (situation/witnesses/evidence/W-000018/msrv-check-1.96.0-recheck-at-f805688.log). |
| P2 | PASS — the manifest at f80568881fccd9e32ad99f593b39b6e54f9fa59a declares `rust-version = "1.96"` (Cargo.toml); the re-anchoring inventory shows the oldest installed stable toolchain is 1.96.0 (situation/witnesses/evidence/W-000018/msrv-toolchain-inventory-at-f805688.log). |
