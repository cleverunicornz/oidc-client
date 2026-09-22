# O-000006 — Judge configured CI gate route

## State

designed

## Judges

situation/promises/P-000005-configured-ci-gate-route.md

## Inputs

`.github/workflows/ci.yml`, the workflow event and runner metadata, and a
retained output from one configured workflow run.

## Pass

- P1: The workflow declares only the configured pull-request routes and manual
  dispatch, and its `ci` job uses `cvu-test-runner-x64`.
- P2: The `ci` job checks the required native capabilities before configuring
  all four named gate commands.
- P3: A retained run of that job executes every named command successfully on
  the configured route.

## Fail

- F1: The workflow route or runner differs from the configured boundary.
- F2: A required capability check or gate command is absent from the configured
  job.
- F3: A retained configured run omits, cannot execute, or fails any named
  command.
