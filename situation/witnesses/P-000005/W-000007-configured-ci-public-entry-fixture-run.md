# W-000007 — Passing configured CI run after the public-entry fixture correction

## Promise

situation/promises/P-000005-configured-ci-gate-route.md

## Oracle

situation/oracles/O-000006-judge-configured-ci-gate-route.md

## Result

PASS — the configured `ci` job was explicitly dispatched on `bank2/assurance`
and completed successfully after the public-entry fixture correction.

## Head

38796c0752cc294bd58eafdac17e361d4f88ac7b

## Observed

2026-09-22

## Evidence

- https://github.com/cleverunicornz/oidc-client/actions/runs/35745613778 —
  GitHub run metadata reports event `workflow_dispatch`, head
  `38796c0752cc294bd58eafdac17e361d4f88ac7b`, and conclusion `success`.
- `38796c0752cc294bd58eafdac17e361d4f88ac7b:.github/workflows/ci.yml`
  declares the configured same-repository pull-request routes and
  `workflow_dispatch`, the `cvu-test-runner-x64` runner, the required
  native-capability step, and the four named gate commands.
- The run's job metadata reports success for "Required native capabilities",
  "Formatting", "Clippy", "Tests", and "Dependency policy" in that order.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the stated head declares the configured routes, repository guard, and `cvu-test-runner-x64`; run 35745613778 was a successful manual dispatch. |
| P2 | PASS — the run reports the required native-capability step succeeded before the four gate-command steps. |
| P3 | PASS — the run reports Formatting, Clippy, Tests, and Dependency policy each succeeded. |
