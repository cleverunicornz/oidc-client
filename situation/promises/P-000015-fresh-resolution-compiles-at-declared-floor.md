# P-000015 — Fresh resolution compiles at the declared MSRV floor

## State

implemented

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

`implemented` — the manifest declares `rust-version = "1.96"` and the
proof battery was recorded at commit
e0860535f99aa50ad3d45befcea7ed1644864cc0 on branch
`fix/prepublication-hardening` (W-000018).

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
