# W-000004 — Passing configured fleet run of the `ci` job

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

PASS — one explicit `workflow_dispatch` execution of the configured `ci` job
on `bank2/assurance` completed successfully at the head below.

## Head

7fe8265c8166e16ea4da77b5722a35503fa67665

## Observed

2026-09-22

## Evidence

- https://github.com/cleverunicornz/oidc-client/actions/runs/35743515109 —
  GitHub run metadata reports event `workflow_dispatch`, ref
  `bank2/assurance`, head
  `7fe8265c8166e16ea4da77b5722a35503fa67665`, and conclusion `success`
  (run database id 35743515109; job id 106799076856, 55s).
- `7fe8265c8166e16ea4da77b5722a35503fa67665:.github/workflows/ci.yml`
  declares the configured same-repository pull-request routes and
  `workflow_dispatch`, the `cvu-test-runner-x64` runner, the required
  native-capability step, and the four named gate commands.
- The run's job metadata reports success for "Required native capabilities",
  "Formatting", "Clippy", "Tests", and "Dependency policy" in that order.
- This Witness is bounded to run 35743515109 only. Its head predates the
  public-entry fixture correction recorded in W-000003 and therefore does not
  corroborate that Witness; later and failed CI observations have their own
  Witness records.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the stated head declares the configured routes, repository guard, and `cvu-test-runner-x64`; run 35743515109 was a successful manual dispatch. |
| P2 | PASS — the run reports the required native-capability step succeeded before the four gate-command steps. |
| P3 | PASS — the run reports Formatting, Clippy, Tests, and Dependency policy each succeeded. |
