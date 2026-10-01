# O-000027 — Judge CI route that supersedes pull-request runs

## State

implemented

## Judges

situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md

## Inputs

- `.github/workflows/ci.yml` at the judged head.
- The workflow runs of a test pull request, read with the github MCP
  `actions_read` `list_runs` (name, event, head_sha, status, conclusion, URL)
  and the run's job metadata, after two normal commits (no `[skip ci]`) are
  pushed a few seconds apart and after the pull request is closed and
  reopened twice in quick succession.

## Pass

- P1: The workflow declares the pull-request types `[opened, reopened,
  ready_for_review]` with `paths-ignore` exactly `'**.md'`, `'docs/**'`,
  `'situation/**'`, `'LICENSE*'`, and an unfiltered `workflow_dispatch`; the
  `ci` job's guard admits only same-repository pull requests or manual
  dispatch; the job runs on runner group `ci`, label `automation-test-s`.
- P2: The job checks the required native capabilities before the four named
  gate commands.
- P3: The workflow carries, at workflow level, exactly the block of
  situation/decisions/D-000020-ci-route-supersedes-pull-request-runs-and-skips-docs-only-changes.md.
  This leg decides Promise item 4.
- P4: Of the two `reopened` runs at one head, the first ends `cancelled`; the
  second runs every named command successfully on the configured runner.
- P5: The two pushes start no run.

## Fail

- F1: The routes, path filter, guard or runner differ from P1.
- F2: A capability check or gate command is absent, or out of order.
- F3: The block is absent, differs, or sits at job level only.
- F4: The first `reopened` run is not `cancelled`, the second is `cancelled`,
  or the second omits or fails a named command.
- F5: A push starts a run.
- F6: A dispatch run in the observed window ends `cancelled` through
  concurrency.

## Implementation

The `.github/workflows/ci.yml` `ci` job executes the capability checks and
gate commands; its exit status decides the run result used by P4.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Routes, filter, guard and runner compared with the declared boundary | manual |
| P2 | Step order and named commands compared with the declared boundary | manual |
| P3 | Block present and exact at workflow level | manual |
| P4 | First reopen run cancelled; second runs every command successfully | `.github/workflows/ci.yml` `ci` job (commands); manual (`actions_read` `list_runs` for cancel) |
| P5 | Pushes start no run | manual (`actions_read` `list_runs`) |
| F1 | Route, filter, guard or runner difference | manual |
| F2 | Missing or reordered capability check or command | manual |
| F3 | Block absent or different | manual |
| F4 | Live supersede or command result does not hold | `.github/workflows/ci.yml` `ci` job; manual |
| F5 | A push started a run | manual (`actions_read` `list_runs`) |
| F6 | A dispatch run cancelled | manual (`actions_read` `list_runs`) |
