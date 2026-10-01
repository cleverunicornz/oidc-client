# P-000017 — Configured CI route runs the gate commands, superseding older pull-request runs

## State

assured

## Promise

1. The configured GitHub Actions `ci` job runs the required
   native-capability checks, then `cargo fmt --all --check`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test --all-features`
   and `cargo deny check`, on runner group `ci`, label `automation-test-s`,
   for same-repository pull requests on `opened`, `reopened` and
   `ready_for_review` and for manual dispatch.
2. A pull request whose changed files all match `**.md`, `docs/**`,
   `situation/**` or `LICENSE*` starts no pull-request run.
3. When a `pull_request` event starts a run while an earlier `pull_request`
   run of the same pull request is queued or running, the earlier run ends
   `cancelled` and the newer run runs to its own conclusion.
4. A manual-dispatch run is never cancelled or replaced through concurrency,
   and a `pull_request` run never cancels a run of another pull request.

## Scope

The single `ci` job in `.github/workflows/ci.yml`: its declared triggers and
path filter, its repository guard
(`github.event.pull_request.head.repo.full_name == github.repository`, or
`workflow_dispatch`), its workflow-level concurrency block, its runner, its
capability checks and four gate commands. Fork pull requests never run the
job. This Promise supersedes
situation/promises/P-000005-configured-ci-gate-route.md, and is this
repository's part of the organization contract
`Private: cleverunicornz/infra-v2@ci/cancel-superseded-pr-runs#situation/promises/P-000087-pull-request-runs-supersede-only-their-own.md`
(private repository; cleverunicornz/infra-v2 pull request #488).

## Oracle

situation/oracles/O-000027-judge-ci-route-supersedes-pull-request-runs.md

## State evidence

- Implementation: commit `1d0fa93ec5473f9bb41e6afe70283344499cd762`
  (pull request https://github.com/cleverunicornz/oidc-client/pull/9).
- Decision:
  situation/decisions/D-000020-ci-route-supersedes-pull-request-runs-and-skips-docs-only-changes.md
- `assured`: O-000027 passes on
  situation/witnesses/P-000017/W-000040-ci-route-supersedes-pull-request-runs.md

## Residual

- A future runner environment is not assured; a capability regression fails
  the gate.
- Item 2 is decided by the declared filter; no docs-only pull request was
  observed.
- Item 4 is decided by the block itself (a group keyed on `run_id` with
  `cancel-in-progress` false); no dispatch was observed in the witness window.
- A push to a pull request starts no run (no `synchronize` type), so item 3
  arises only between `opened`, `reopened` and `ready_for_review` runs.

## References

- situation/promises/P-000005-configured-ci-gate-route.md (superseded route)
- situation/decisions/D-000020-ci-route-supersedes-pull-request-runs-and-skips-docs-only-changes.md
