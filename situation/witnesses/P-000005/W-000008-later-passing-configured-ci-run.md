# W-000008 — Later passing configured CI run

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

PASS — the configured `ci` job was explicitly dispatched on `bank2/assurance`
and completed successfully at the head below.

## Head

5dbfc4b46204ca7c36c39283103e4efbe88e1c56

## Observed

2026-09-22

## Evidence

- https://github.com/cleverunicornz/oidc-client/actions/runs/35746649645 —
  GitHub run metadata reports event `workflow_dispatch`, head
  `5dbfc4b46204ca7c36c39283103e4efbe88e1c56`, and conclusion `success`.
- `5dbfc4b46204ca7c36c39283103e4efbe88e1c56:.github/workflows/ci.yml`
  declares the configured same-repository pull-request routes and
  `workflow_dispatch`, the `cvu-test-runner-x64` runner, the required
  native-capability step, and the four named gate commands.
- The run's job metadata reports success for "Required native capabilities",
  "Formatting", "Clippy", "Tests", and "Dependency policy" in that order.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the stated head declares the configured routes, repository guard, and `cvu-test-runner-x64`; run 35746649645 was a successful manual dispatch. |
| P2 | PASS — the run reports the required native-capability step succeeded before the four gate-command steps. |
| P3 | PASS — the run reports Formatting, Clippy, Tests, and Dependency policy each succeeded. |
