# O-000006 — Judge configured CI gate route

## State

implemented

## Judges

situation/promises/P-000005-configured-ci-gate-route.md

## Inputs

`.github/workflows/ci.yml`, the workflow event and runner metadata, and a
retained output from one configured workflow run.

## Pass

- P1: The workflow declares only the configured pull-request routes
  `[opened, reopened, ready_for_review]` and manual dispatch, its `ci` job
  guard admits only same-repository pull requests
  (`github.event.pull_request.head.repo.full_name == github.repository`) or
  manual dispatch, and the job uses `cvu-test-runner-x64`.
- P2: The `ci` job checks the required native capabilities before configuring
  all four named gate commands.
- P3: A retained run of that job executes every named command successfully on
  the configured route.

## Fail

- F1: The workflow routes, the `ci` job's repository guard, or the runner
  differs from the configured boundary.
- F2: A required capability check or gate command is absent from the configured
  job.
- F3: A retained configured run omits, cannot execute, or fails any named
  command.

## Implementation

The `.github/workflows/ci.yml` `ci` job executes the configured capability
checks and gate commands. Its command results are recorded as individual
workflow-run Witnesses under P-000005.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | The YAML routes, guard, and runner are compared with the declared boundary. | manual |
| P2 | The YAML step order and named commands are compared with the declared boundary. | manual |
| P3 | The `ci` job runs each named command and its exit status decides the configured run result. | `.github/workflows/ci.yml` `ci` job |
| F1 | A route, guard, or runner difference is identified by YAML comparison. | manual |
| F2 | A missing capability check or command is identified by YAML comparison. | manual |
| F3 | Retained run metadata is checked for omitted, unexecuted, or failed commands. | manual |

## Provenance

Corrected in place on open PR #2 before any closing checkpoint (CodeRabbit
thread PRRT_kwDOUlFWIM6kzMgq): P1 and F1 now name the `ci` job's fork guard
(`github.event.pull_request.head.repo.full_name == github.repository`),
verified against `.github/workflows/ci.yml` at the review head.
