# G-000035 — The build trigger model's workflow shape and merge gate are checked by hand here, not by CI

## State

open

## Gap

- No CI job of this repository runs infra-v2's `check-workflow-concurrency` checker, so a later workflow change could
  drop `synchronize` or add a heavy job without the model's shape, and no check would fail. The checker lives in a
  private repository whose files another repository's `GITHUB_TOKEN` cannot fetch; it is not vendored here.
- The merge gate is a procedure (infra-v2 G-000260): GitHub does not require it.
- [P-000005](situation/promises/P-000005-configured-ci-gate-route.md) is superseded by
  [P-000017](situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md) (P-000017 says so; commit
  `593979e`), but its State still reads `assured` and its Scope still names the runner `cvu-test-runner-x64` and the
  routes `[opened, reopened, ready_for_review]`. Observed while superseding P-000017 here; not changed by this work.

## Relevance

[P-000018](situation/promises/P-000018-ci-runs-on-every-push.md), [D-000021](situation/decisions/D-000021-ci-runs-on-every-push.md).

## Evidence

- Observation: `.github/workflows/` at the implementing head has no job running `check-workflow-concurrency.py`.
- Interpretation: drift is caught only when someone runs the pinned checker by hand.

## Impact

A workflow edit could take light checks off pushes, or add an unguarded heavy job, and still merge.

## Resolution

none
