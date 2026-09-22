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

situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000004
(project Status: Todo). Corrected against repository law: the project text
says CI runs "on every push and PR"; per the root AGENTS.md workflow
policy, branches carry no push triggers — CI runs when a pull request
opens and on its final head by dispatch.
