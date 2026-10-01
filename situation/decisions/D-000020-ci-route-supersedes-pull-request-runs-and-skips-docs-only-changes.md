# D-000020 — CI route supersedes pull-request runs and skips docs-only changes

## Status

accepted

## Date

2026-10-01

## Context

- situation/promises/P-000005-configured-ci-gate-route.md (assured) states
  that the `ci` job runs its capability checks and four gate commands for the
  declared same-repository pull-request routes and manual dispatch, on
  `cvu-test-runner-x64`.
- Since commit `bcc0285` ("ci: move jobs to the in-cluster runner sets") the
  job runs on runner group `ci`, label `automation-test-s`; the retired
  `cvu-test-runner-x64` label matches no runner. P-000005 was not superseded
  at that change.
- The organization rule (PLATFORM-PLAN P6, 2026-10-01) is that every
  pull-request-triggered workflow cancels a superseded run of the same pull
  request and workflow, while other runs are never cancelled half-way, and
  that a pull-request workflow with no path filter skips changes its jobs
  cannot be affected by. It is decided once for the organization in
  `Private: cleverunicornz/infra-v2@ci/cancel-superseded-pr-runs#situation/decisions/D-000203-pull-request-runs-supersede-only-their-own.md`
  (private repository; cleverunicornz/infra-v2 pull request #488), and stated
  as the composing contract
  `Private: cleverunicornz/infra-v2@ci/cancel-superseded-pr-runs#situation/promises/P-000087-pull-request-runs-supersede-only-their-own.md`.
- Adding either rule changes P-000005's declared route, so it takes the
  supersession path.

## Evidence

- `9e511f962be3bfb18e02ada87057bda0614e1246:.github/workflows/ci.yml`: no
  `concurrency`, no path filter, runner group `ci`, label
  `automation-test-s`.
- No file under `src/`, `tests/` or `examples/`, and neither `Cargo.toml` nor
  `deny.toml`, includes a Markdown, `docs/`, `situation/` or `LICENSE*` file
  into a compiled or checked input (`grep -rn "include_str\|include_bytes"
  src tests examples` returns nothing); `Cargo.toml` names `README.md` only as
  package metadata.
- GitHub concurrency semantics:
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#concurrency
- The live observation on this change's pull request:
  situation/witnesses/P-000017/W-000040-ci-route-supersedes-pull-request-runs.md

## Decision

1. `.github/workflows/ci.yml` carries, at workflow level, exactly:

   ```yaml
   concurrency:
     group: ${{ github.workflow }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}
     cancel-in-progress: ${{ github.event_name == 'pull_request' }}
   ```

2. The `pull_request` trigger ignores `'**.md'`, `'docs/**'`, `'situation/**'`
   and `'LICENSE*'`; `workflow_dispatch` stays unfiltered.
3. situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md
   supersedes P-000005, judged by
   situation/oracles/O-000027-judge-ci-route-supersedes-pull-request-runs.md,
   which replaces O-000006. The new Promise states the actual runner.
4. `on.pull_request.types` stays `opened`, `reopened`, `ready_for_review`.

## Why

- A `pull_request` run gets the group `CI-<PR number>` and cancels the older
  run of the same pull request (a `reopened` or `ready_for_review` run started
  while an earlier one is still queued or running).
- A dispatch run gets a group keyed on its own `run_id`, which no other run
  shares, and `cancel-in-progress` is false for it.
- None of the ignored paths is read by fmt, clippy, the tests or cargo deny,
  so a pull request confined to them cannot change the gate's result.

## Rejected alternatives

- Add the block without a path filter: allowed, but leaves record-only and
  documentation-only pull requests building the crate for nothing.
- Correct P-000005 in place: it is assured and merged; changing it takes the
  supersession path.
- Add `synchronize` to the pull-request types: a change of which pushes start
  a run, outside this decision; without it a push starts no run.

## Consequences

- P-000005, O-000006 and their witnesses remain as history for the earlier
  route.
- A pull request's checks show `cancelled` for a superseded run; only the
  newest run counts. No ruleset requires a status check.

## Revisit when

- The organization changes the block decided in infra-v2 D-000203.
- A Markdown, `docs/`, `situation/` or `LICENSE*` file becomes a compiled,
  tested or policy-checked input.
