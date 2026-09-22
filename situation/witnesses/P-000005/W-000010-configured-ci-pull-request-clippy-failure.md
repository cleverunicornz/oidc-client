# W-000010 — Failed configured CI pull-request run

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

FAIL — the configured `ci` job ran for the same-repository pull-request event,
but its Clippy step failed at the head below.

## Head

f0ee1c29fa32224b24487dc5322a8c6fc2b2535b

## Observed

2026-09-22

## Evidence

- https://github.com/cleverunicornz/oidc-client/actions/runs/35743042564 —
  GitHub run metadata reports event `pull_request`, head
  `f0ee1c29fa32224b24487dc5322a8c6fc2b2535b`, and conclusion `failure`.
- `f0ee1c29fa32224b24487dc5322a8c6fc2b2535b:.github/workflows/ci.yml`
  declares the configured same-repository pull-request routes and
  `workflow_dispatch`, the `cvu-test-runner-x64` runner, the required
  native-capability step, and the four named gate commands.
- The run's job metadata reports success for "Required native capabilities"
  and "Formatting", failure for "Clippy", and success for "Tests" and
  "Dependency policy".

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the stated head admits same-repository pull-request events and declares `cvu-test-runner-x64`; run 35743042564 was a pull-request event. |
| P2 | PASS — the run reports the required native-capability step succeeded before the four gate-command steps. |
| P3 | FAIL — the run reports the Clippy command failed, so not every named command executed successfully. |
