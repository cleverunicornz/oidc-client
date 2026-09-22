# P-000005 — Configured CI route runs repository gate commands

## State

assured

## Promise

The configured GitHub Actions `ci` job runs required native-capability checks,
then `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-features`, and `cargo deny check` on
`cvu-test-runner-x64` for its declared pull-request routes and manual
dispatch.

## Scope

The single `ci` job in `.github/workflows/ci.yml`, its declared triggers,
runner, capability checks, and four gate commands. This promise does not
assure that a future runner retains those capabilities, or that a package
has been published.

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## State evidence

State `assured` cites Oracle
situation/oracles/O-000006-judge-configured-ci-gate-route.md and the PASS
witness
situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md: the
dispatched run
https://github.com/cleverunicornz/oidc-client/actions/runs/35743515109
(head `7fe8265c8166e16ea4da77b5722a35503fa67665` on `bank2/assurance`)
executed the capability checks and all four named gate commands
successfully on `cvu-test-runner-x64`. The configuration was added by
commit `8160e19c82e76aadc967ce470d4a4285da5c1617`.
A corroborating green dispatch at the branch's final head
(`5dbfc4b46204ca7c36c39283103e4efbe88e1c56`, run 35746649645) and an
intermediate one at `38796c0` (run 35745613778, first fleet execution of the
corrected public-entry fixture) are retained in the same witness.

## Residual

A future runner environment is not assured; a capability regression will
fail the configured gate. Local or pinned-container gate runs remain
non-workflow evidence and do not assure this promise. The workflow's
non-normative test-count comment at `.github/workflows/ci.yml` line 43
("70 offline unit + doc tests"; the retained run executes 72 unit tests
plus 7 passing and 2 ignored doctests) remains stale, as recorded in
G-000004's evidence.

## References

- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md
- situation/decisions/D-000009-promote-configured-ci-gate-route.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md — closed by the
  retained witness above.
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
