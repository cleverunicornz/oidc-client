# P-000005 — Configured CI route runs repository gate commands

## State

implemented

## Promise

The configured GitHub Actions `ci` job runs required native-capability checks,
then `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-features`, and `cargo deny check` on
`cvu-test-runner-x64` for its declared pull-request routes and manual
dispatch.

## Scope

The single `ci` job in `.github/workflows/ci.yml`, its declared triggers,
runner, capability checks, and four gate commands. This promise does not assure
that any workflow run has succeeded, that a future runner has the capabilities,
or that a package has been published.

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## State evidence

State `implemented` is supported by commit
`8160e19c82e76aadc967ce470d4a4285da5c1617`, which added the configured
workflow route. G-000004 records that no retained workflow-run Witness yet
applies O-000006.

## Residual

The first configured fleet-run observation remains absent. Local or
pinned-container gates are not workflow evidence and do not assure this
promise.

## References

- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md
- situation/decisions/D-000009-promote-configured-ci-gate-route.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
