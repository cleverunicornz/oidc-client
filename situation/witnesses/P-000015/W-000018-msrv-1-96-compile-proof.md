# W-000018 — Fresh-resolution compile proof on 1.96.0 and 1.98.0

## Promise

situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md

## Oracle

situation/oracles/O-000016-judge-fresh-resolution-compiles-at-declared-floor.md

## Result

PASS — every verified-set toolchain (1.96.0, 1.98.0) exited 0 on
fresh-resolution `cargo check` for the default feature set and for
`--all-features` at the head below, and the declared floor equals the
oldest installed stable toolchain.

## Head

e0860535f99aa50ad3d45befcea7ed1644864cc0

## Observed

2026-09-23

## Evidence

- `situation/witnesses/evidence/W-000018/msrv-check-1.96.0.log` retains
  the full verbatim 1.96.0 battery: `rustc +1.96.0 -vV` (release
  1.96.0), then `rm -f Cargo.lock && cargo +1.96.0 check --offline` and
  `rm -f Cargo.lock && cargo +1.96.0 check --offline --all-features`,
  each `Finished \`dev\` profile` with `exit=0`. Resolution was fresh —
  no `Cargo.lock` existed before either command.
- `situation/witnesses/evidence/W-000018/msrv-check-1.96.0-recheck.log`
  retains `touch src/lib.rs && cargo +1.96.0 check --offline
  --all-features` at the same head — a forced fresh rustc invocation on
  the crate (`Checking oidc-client v4.1.0` … `Finished`, `exit=0`),
  corroborating that 1.96.0 compiles the final manifest itself, not only
  the cached dependency artifacts.
- `situation/witnesses/evidence/W-000018/msrv-check-1.98.0.log` retains
  the 1.98.0 battery: both fresh-resolution commands end
  `Finished \`dev\` profile` with `exit=0`, with the crate re-checked
  (`Checking oidc-client v4.1.0`).
- `situation/witnesses/evidence/W-000018/msrv-check-nightly-informational.log`
  retains the informational nightly run (1.100.0-nightly,
  `--all-features`, `exit=0`); nightly is outside the verified set.
- `situation/witnesses/evidence/W-000018/msrv-toolchain-inventory.log`
  retains `rustup toolchain list` at proof time: nightly-x86_64-unknown-linux-gnu,
  1.96.0-x86_64-unknown-linux-gnu,
  1.98.0-x86_64-unknown-linux-gnu (active, default) — `exit=0`.
- `situation/witnesses/evidence/W-000018/SHA256SUMS` records SHA-256
  digests for the five retained files.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — 1.96.0 and 1.98.0 both exit 0 on both fresh-resolution commands (msrv-check-1.96.0.log, msrv-check-1.98.0.log); the forced 1.96.0 recheck confirms a real rustc invocation on the crate at the final manifest (msrv-check-1.96.0-recheck.log). |
| P2 | PASS — the manifest at the head declares `rust-version = "1.96"`; the inventory shows the oldest installed stable toolchain is 1.96.0 (msrv-toolchain-inventory.log). |
