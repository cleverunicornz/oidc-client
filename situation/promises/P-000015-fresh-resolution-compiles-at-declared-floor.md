# P-000015 — Fresh resolution compiles at the declared MSRV floor

## State

assured

## Promise

A fresh resolution of this crate's manifest — no `Cargo.lock` present,
resolution from the crates.io index — compiles with `cargo check` under
every stable toolchain in the locally verified set whose release is >=
the manifest's declared `rust-version` (currently "1.96"), for the
default feature set and for `--all-features`.

## Scope

- The declared floor is the `rust-version` field of the root
  `Cargo.toml`.
- The locally verified set is the stable rustup toolchains installed on
  the verification host at observation time; at creation these are
  1.96.0 and 1.98.0.
- Fresh resolution means each command runs with no `Cargo.lock` present;
  the offline registry cache is a valid resolution source.
- "Compiles" means the `cargo check` command exits 0.

## Oracle

situation/oracles/O-000016-judge-fresh-resolution-compiles-at-declared-floor.md

## State evidence

`assured` — O-000016 passed on W-000018's 2026-09-24 battery at
reachable head f80568881fccd9e32ad99f593b39b6e54f9fa59a on branch
`fix/prepublication-hardening` (P1: both verified-set toolchains exit 0
on both fresh-resolution commands; P2: the declared 1.96 floor equals
the oldest installed stable); the original 2026-09-23 battery at
e0860535f99aa50ad3d45befcea7ed1644864cc0 (unresolvable on the branch,
G-000029) is retained in W-000018 as history.

## Residual

- Toolchains older than 1.96 are outside this promise: the dependency
  graph's true lower floor is unproven (G-000026) and no behavior is
  claimed for it.
- Nightly and beta toolchains are outside the verified set; the nightly
  run retained in W-000018 is information only, not assurance.
- CI pins no MSRV job (G-000026); this promise is judged from recorded
  witnesses, not continuously enforced.

## References

- situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md
- situation/gaps/G-000026-true-msrv-floor-unproven-and-no-msrv-ci-job.md
