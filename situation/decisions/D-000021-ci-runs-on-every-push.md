# D-000021 — CI is a light check of the build trigger model: it runs on every push to a pull request

## Status

accepted

## Date

2026-10-02

## Context

- The organization's build trigger model (operator ruling of 2026-10-01): light checks (lint, unit tests, record
  checks) run on every pull request push; heavy builds run only on `ready_for_review`, a `build` label and
  `workflow_dispatch`; nothing merges without a green heavy build of the exact head. Decided in infra-v2 D-000210,
  stated as infra-v2 P-000095: `Private: cleverunicornz/infra-v2@0f6aa5cc#situation/promises/P-000095-build-trigger-model.md`
(https://github.com/cleverunicornz/infra-v2/blob/0f6aa5cc/situation/promises/P-000095-build-trigger-model.md;
organization access; [#498](https://github.com/cleverunicornz/infra-v2/pull/498)). D-000210 item 9 rolls it out to each active repository by its own pull
  request; its item 2 gives light workflows `synchronize`.
- D-000020 gave `CI` the organization's supersession block and docs-only skip, and left adding `synchronize` ("a change of which pushes start a run") outside itself; this is that decision.

## Evidence

- `.github/workflows/ci.yml` at `82ffdfe`: `types: [opened, reopened, ready_for_review]`, `paths-ignore` (`**.md`, `docs/**`, `situation/**`, `LICENSE*`), D-000020's block, one job `ci` on automation-test-s (capability checks, fmt, clippy, tests, deny) behind the repository/fork guard.
- infra-v2's checker `scripts/check-workflow-concurrency.py` at `0f6aa5cc` classifies the workflow as light and fails
  it at `82ffdfe` on one point: its `pull_request` types leave out `synchronize`.
- The live proof: [W-000041](situation/witnesses/P-000018/W-000041-ci-runs-on-every-push.md).

## Decision

1. `.github/workflows/ci.yml` adds `synchronize` to its `pull_request` types: `[opened, reopened, ready_for_review,
   synchronize]`. Every push to a pull request the workflow admits starts a run.
2. D-000020's concurrency block, `paths-ignore`, the repository/fork guard and the job's steps are unchanged.
3. This repository has no heavy `pull_request` workflow (no job on build-docker, automation-test-xl or build-native),
   so the model's heavy shape does not apply. The repository label `build` exists for the model's procedure.
4. Judged by hand with infra-v2's checker pinned at `0f6aa5cc` (G-000035).

## Why

- The job is a light check (fmt, clippy, tests, cargo deny) on a 5 GiB automation-test-s slot; the model wants its feedback on every push, and
  the existing block cancels the superseded push's run.

## Rejected alternatives

- **Keep pushes off** (no `synchronize`): against the model's light-checks-on-every-push rule.
- **Move the job to a heavy runner and request it with the label:** it is not a heavy build.

## Consequences

- [P-000018](situation/promises/P-000018-ci-runs-on-every-push.md) states the behaviour and supersedes
  [P-000017](situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md); [O-000028](situation/oracles/O-000028-ci-runs-on-every-push.md) judges it.
- Before merge, infra-v2's `scripts/heavy-build-green.sh cleverunicornz/oidc-client <number>` must exit 0: with no heavy
  workflow, that is a green run of this workflow on the exact head.

## Revisit when

A heavy `pull_request` workflow is added here, or the platform model changes.
