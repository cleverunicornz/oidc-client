# C-000004 — CI pipeline (fmt, clippy, test, audit)

## State

qualifying — the pipeline is landed as .github/workflows/ci.yml per the
organization workflow policy (root AGENTS.md organization block:
pull_request types [opened, reopened, ready_for_review] + workflow_dispatch,
no push trigger; fleet runner labels) and the lane decisions in
situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
(pinned rust:1.98.0 container, clippy remediation policy), with all four
gates green in the pinned container
(situation/witnesses/evidence/W-000001/gates-final.log). The plan
(situation/plans/active/PLAN-000001-import-es256-publish-consume.md) keeps
the candidate under qualification until the workflow's first real run on
the fleet runner proves it; once proven, the gates continuously judge leg
P6 of
situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md
on
situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md.
The candidates law promotes only into a Decision + Promise + Oracle, and
no dedicated pair was created for the pipeline itself.

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
