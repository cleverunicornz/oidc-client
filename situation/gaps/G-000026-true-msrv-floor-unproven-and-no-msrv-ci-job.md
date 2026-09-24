# G-000026 — True MSRV floor unproven and CI pins no MSRV job

## State

open

## Gap

Two absences around the declared 1.96 floor:

1. The dependency graph's true lower MSRV floor is unproven below the
   compilation-verified 1.96. The resolved set's declared minimums
   (ed25519-dalek 2.2.0 at 1.81, base64ct 1.8.3 at 1.85, serde_with
   3.23.0 at 1.88) describe only the current fresh resolution and prove
   nothing about what compiles on older toolchains, whose resolver may
   select older dependency versions; no toolchain older than 1.96.0 is
   installed locally, and installing additional toolchains locally is
   prohibited by fleet policy — a missing capability is requested, not
   installed. The declared 1.96 floor may therefore be stricter than
   necessary; nothing currently verifies that.
2. CI pins no MSRV job. `.github/workflows/ci.yml` has a single `ci`
   job (required capabilities, formatting, clippy, tests, dependency
   policy) on the pinned gate toolchain; no job compiles the crate at
   the declared `rust-version`. Whether the fleet offers the toolchain
   capability an MSRV job would need is unverified.

## Relevance

The MSRV claim surface: `Cargo.toml`'s `rust-version`, the README
"Minimum supported Rust version" section, and P-000015, whose promise is
judged from recorded witnesses rather than continuously enforced.

## Evidence

- Observation: `rustup toolchain list` at proof time shows only
  1.96.0, 1.98.0, and nightly (retained in
  situation/witnesses/evidence/W-000018/msrv-toolchain-inventory.log);
  the declared-minimum readings come from each dependency's published
  manifest in the local registry cache
  (`serde_with-3.23.0` → 1.88, `ed25519-dalek-2.2.0` → 1.81,
  `base64ct-1.8.3` → 1.85).
- Observation: the `ci.yml` step list (capabilities, formatting,
  clippy, tests, dependency policy) contains no toolchain pinned to the
  declared MSRV.
- Interpretation: the floor below the compilation-verified 1.96 is
  unproven — the declared minimums are facts about the current
  resolution, not compilation evidence, and no compilation evidence for
  older toolchains exists (D-000016 forbids metadata-inferred floor
  claims).

## Impact

Consumers building on toolchains below the declared 1.96 have no
verified support statement, so the crate may be rejecting builds that
would succeed. A regression that silently raises the real floor above
the declared one would be caught only by re-running the P-000015
battery by hand, not by CI.

## Resolution

none

## References

- situation/decisions/D-000016-msrv-baseline-is-compilation-proven.md
- situation/promises/P-000015-fresh-resolution-compiles-at-declared-floor.md
- situation/oracles/O-000016-judge-fresh-resolution-compiles-at-declared-floor.md
- situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md
