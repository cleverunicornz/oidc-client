# O-000003 — Judge crates.io publication

## State

designed

## Judges

situation/promises/P-000003-published-on-crates-io-as-oidc-client.md

## Inputs

An empty Rust project consuming the published crate, the crates.io crate
page, and the repository's CI configuration and its runs.

## Pass

- P1: cargo add oidc-client succeeds from an empty Rust project
- P2: crates.io page shows documentation, repository, license
- P3: CI runs the fmt, clippy, test, and audit checks on the pull request —
  once when it opens and once on its final head by dispatch
- P4: Version is semantic (0.1.0 for initial release or 4.1.0 to signal
  continuity)

## Fail

- F1: Crate not found on crates.io
- F2: CI does not run

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item O-000003
(project Status: Todo). Corrected against repository law: the project body
says "CI runs on push and PR"; per the root AGENTS.md workflow policy,
branches carry no push triggers — CI runs when a pull request opens and on
its final head by dispatch.
