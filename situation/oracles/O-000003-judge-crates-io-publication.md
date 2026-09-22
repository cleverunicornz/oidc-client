# O-000003 — Judge crates.io publication

## State

designed

## Judges

P-000003

## Inputs

An empty Rust project consuming the published crate, the crates.io crate
page, and the repository's CI configuration and its runs.

## Pass

- P1: cargo add oidc-client succeeds from an empty Rust project
- P2: crates.io page shows documentation, repository, license
- P3: CI runs on push and PR (fmt, clippy, test, audit)
- P4: Version is semantic (0.1.0 for initial release or 4.1.0 to signal
  continuity)

## Fail

- F1: Crate not found on crates.io
- F2: CI does not run

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item O-000003
(project Status: Todo).
