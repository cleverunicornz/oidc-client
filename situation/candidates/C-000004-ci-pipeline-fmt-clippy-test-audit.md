# C-000004 — CI pipeline (fmt, clippy, test, audit)

## State

proposed

## What

Set up GitHub Actions CI that runs on every push and PR: cargo fmt --check,
cargo clippy -- -D warnings, cargo test, cargo audit / cargo deny check.
Use the fleet runner labels.

## Approach

1. Create .github/workflows/ci.yml with the four jobs
2. Use the appropriate fleet runner (select-runner skill)
3. Ensure the test job runs both unit and integration tests
4. cargo-deny for license and advisory checking

## Dependencies

C-000001

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000004
(project Status: Todo).
