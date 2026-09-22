# P-000005 — Configured CI route runs repository gate commands

## State

assured

## Promise

The configured GitHub Actions `ci` job runs required native-capability checks,
then `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-features`, and `cargo deny check` on
`cvu-test-runner-x64` for its declared pull-request routes (same-repository
pull requests only) and manual dispatch.

## Scope

The single `ci` job in `.github/workflows/ci.yml`, its declared triggers,
runner, capability checks, and four gate commands. This promise does not
assure that a future runner retains those capabilities, or that a package
has been published.

The job's `if:` guard admits it only for same-repository pull requests
(`github.event.pull_request.head.repo.full_name == github.repository`) on
the declared routes `[opened, reopened, ready_for_review]`, or for manual
`workflow_dispatch`; fork pull requests never run this job.

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## State evidence

State `assured` cites Oracle
situation/oracles/O-000006-judge-configured-ci-gate-route.md and the primary
PASS witness
situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md: run
35743515109 at head `7fe8265c8166e16ea4da77b5722a35503fa67665` executed the
capability checks and all four named gate commands successfully on
`cvu-test-runner-x64`. The configuration was added by commit
`8160e19c82e76aadc967ce470d4a4285da5c1617`.

Separate retained observations have their own boundaries: W-000007 records the
PASS dispatch at `38796c0752cc294bd58eafdac17e361d4f88ac7b`, W-000008 records
the PASS dispatch at `5dbfc4b46204ca7c36c39283103e4efbe88e1c56`, W-000009
retains the failed dispatch at `c0d702eb0968f174871f5932ad7d022c3d9ab4be`,
and W-000010 retains the failed pull-request run at
`f0ee1c29fa32224b24487dc5322a8c6fc2b2535b`. The two passing observations are
manual `workflow_dispatch` evidence; the failed pull-request observation is
retained without being used as affirmative assurance.

## Residual

A future runner environment is not assured; a capability regression will
fail the configured gate. Local or pinned-container gate runs remain
non-workflow evidence and do not assure this promise. The workflow's
non-normative test-count comment at `.github/workflows/ci.yml` line 43
remains stale as recorded by
situation/gaps/G-000021-ci-test-count-comment-is-stale.md.

## References

- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md
- situation/decisions/D-000009-promote-configured-ci-gate-route.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md — closed by the
  bounded workflow witnesses below.
- situation/gaps/G-000021-ci-test-count-comment-is-stale.md
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/witnesses/P-000005/W-000007-configured-ci-public-entry-fixture-run.md
- situation/witnesses/P-000005/W-000008-later-passing-configured-ci-run.md
- situation/witnesses/P-000005/W-000009-configured-ci-dispatch-clippy-failure.md
- situation/witnesses/P-000005/W-000010-configured-ci-pull-request-clippy-failure.md

## Provenance

Corrected in place on open PR #2 before any closing checkpoint (CodeRabbit
thread PRRT_kwDOUlFWIM6kzMgq): the `ci` job's fork-guard eligibility boundary
from `.github/workflows/ci.yml` (`github.event.pull_request.head.repo.full_name
== github.repository` on the declared routes, plus `workflow_dispatch`) is
recorded in Promise and Scope. W-000004, W-000007, W-000008, and W-000009 are
manual-dispatch observations; W-000010 preserves the failed same-branch
pull-request observation.
