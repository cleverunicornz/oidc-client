# R-000001 — Upstream pin: openidconnect-rs 4.0.1

## Subject

The exact upstream coordinate this repository carries forward: the
ramosbugs/openidconnect-rs 4.0.1 release — its commit, release timestamp,
license, dependency pins, and import shape.

## Owner

Cited by D-000001, P-000003, C-000001, C-000005, I-000001, and G-000002.

## Facts

- Upstream: https://github.com/ramosbugs/openidconnect-rs
- Tag `4.0.1` (4.x tags carry no `v` prefix) resolves to commit
  `b639b5d39eac6903238867aeb2b29326502e6b26` (lightweight tag).
- crates.io release of openidconnect 4.0.1: 2025-07-06T22:08:11Z.
- License: MIT-only at the tag. The crates.io license field and the tag's
  `Cargo.toml` both declare `MIT`; the tag carries a single `LICENSE` file
  ("Copyright (c) 2018 David Ramos") and zero Apache-2.0 references across
  its 45 tracked files. The "MIT OR Apache-2.0" dual-license phrasing in
  the Project #20 bodies and this repository's blocks is wrong against this
  evidence; repository-block wording is corrected through PR #1 (agents
  never edit the root blocks).
- Dependency pins at 4.0.1 (verified in the tag's `Cargo.toml`):
  `oauth2 = "5.0.0"`, `rsa = "0.9.2"`, `p256 = "0.13.2"` (also
  `p384 = "0.13.0"`), `http = "1.0"`, `serde = "1.0"`.
- Import shape at the pin: a faithful import surface of 41 files — src 30,
  tests 3, examples 3, plus `Cargo.toml`, `Cargo-1.65.lock`, `LICENSE`,
  `README.md`, `UPGRADE.md`; excludes `.github/`, `.gitignore`,
  `.codecov.yml`. Single crate (no workspace), edition 2021,
  rust-version 1.65, 10 features (default = reqwest + rustls-tls), 70
  offline inline tests plus 21 `#[ignore]`d live-network certification
  tests, dev-dependencies: anyhow, color-backtrace, env_logger,
  pretty_assertions, reqwest, retry.
- The crates.io name `oidc-client` was available (unregistered) as of
  2026-09-22.

## Verification

Verified 2026-09-22 by the Phase A records lane:

- `git ls-remote https://github.com/ramosbugs/openidconnect-rs refs/tags/4.0.1`
  returns `b639b5d39eac6903238867aeb2b29326502e6b26`.
- `https://crates.io/api/v1/crates/oidc-client` returns 404 (name
  available).
- `https://crates.io/api/v1/crates/openidconnect/4.0.1` returns
  `created_at` 2025-07-06T22:08:11Z and license `MIT`.
- `Cargo.toml` fetched at commit `b639b5d` (raw.githubusercontent.com)
  declares `license = "MIT"` and the dependency pins listed under Facts.

Verified 2026-09-22 at pin `b639b5d` by the Phase A upstream-facts scout:

- All 45 tracked files reviewed: single `LICENSE` file with the copyright
  line, `Cargo.toml` license `MIT`, no Apache-2.0 references anywhere —
  closing the license question as MIT-only. The import-shape facts under
  Facts come from the same verification.

## References

- situation/gaps/G-000002-bootstrap-record-corrections.md — the context.md
  discrepancies this pin corrects.

## Provenance

Materialized 2026-09-22 by the Phase A records lane; pin, release, name,
and license facts verified the same day as recorded.
