# W-000009 — Failed configured CI dispatch at the pre-correction head

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

FAIL — the configured `ci` job was explicitly dispatched on `bank2/assurance`,
but its Clippy step failed at the head below.

## Head

c0d702eb0968f174871f5932ad7d022c3d9ab4be

## Observed

2026-09-22

## Evidence

- https://github.com/cleverunicornz/oidc-client/actions/runs/35743127287 —
  GitHub run metadata reports event `workflow_dispatch`, head
  `c0d702eb0968f174871f5932ad7d022c3d9ab4be`, and conclusion `failure`.
- `c0d702eb0968f174871f5932ad7d022c3d9ab4be:.github/workflows/ci.yml`
  declares the configured same-repository pull-request routes and
  `workflow_dispatch`, the `cvu-test-runner-x64` runner, the required
  native-capability step, and the four named gate commands.
- The run's job metadata reports success for "Required native capabilities"
  and "Formatting", failure for "Clippy", and success for "Tests" and
  "Dependency policy".

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the stated head declares the configured routes, repository guard, and `cvu-test-runner-x64`; run 35743127287 was a manual dispatch. |
| P2 | PASS — the run reports the required native-capability step succeeded before the four gate-command steps. |
| P3 | FAIL — the run reports the Clippy command failed, so not every named command executed successfully. |
