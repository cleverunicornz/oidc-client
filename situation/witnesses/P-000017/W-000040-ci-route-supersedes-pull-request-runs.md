# W-000040 — CI route supersedes pull-request runs (reopen pair)

## Promise

situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md

## Oracle

situation/oracles/O-000027-judge-ci-route-supersedes-pull-request-runs.md

## Result

PASS

## Head

aeba47eb5802f9028bbe61b5f2caa08f9cdcca5a

## Observed

2026-10-01

## Evidence

All runs read with the github MCP `actions_read` `list_runs` (branch
`ci/cancel-superseded-pr-runs`, pagination complete) on pull request
https://github.com/cleverunicornz/oidc-client/pull/9; close and reopen times
from `gh api repos/cleverunicornz/oidc-client/issues/9/events`.

- `aeba47eb5802f9028bbe61b5f2caa08f9cdcca5a:.github/workflows/ci.yml` is
  byte-identical to the implementation commit
  `1d0fa93ec5473f9bb41e6afe70283344499cd762:.github/workflows/ci.yml`
  (the two commits between them are empty proof commits).
- Two normal commits were pushed about 5 s apart:
  `3395760696695b97d542551cbac13e6cf9d91eff` (push 1) and
  `aeba47eb5802f9028bbe61b5f2caa08f9cdcca5a` (push 2, 13:01:54Z). Neither
  push started a run: no run exists for `3395760…`
  (`gh api "repos/cleverunicornz/oidc-client/actions/runs?head_sha=3395760696695b97d542551cbac13e6cf9d91eff"`
  → `total_count` 0), and the only runs for `aeba47e…` are the two reopen
  runs below.
- The pull request was closed 13:02:47Z, reopened 13:02:53Z, closed
  13:02:59Z and reopened 13:03:05Z.
  - https://github.com/cleverunicornz/oidc-client/actions/runs/36865937494 —
    event `pull_request`, head `aeba47e…`, created 13:02:58Z (first
    reopen), conclusion `cancelled` (updated 13:03:10Z).
  - https://github.com/cleverunicornz/oidc-client/actions/runs/36865956045 —
    event `pull_request`, head `aeba47e…`, created 13:03:07Z (second
    reopen), conclusion `success`. Job
    https://github.com/cleverunicornz/oidc-client/actions/runs/36865956045/job/110381474600
    ran on runner group `ci`, label `automation-test-s`, runner
    `automation-test-s-9nmqs-runner-p5sx7`; steps "Required native
    capabilities", "Formatting", "Clippy", "Tests" and "Dependency policy"
    each `success`, in that order.
- The `opened` run
  https://github.com/cleverunicornz/oidc-client/actions/runs/36865643355
  (head `1d0fa93…`, in progress when the PR was first reopened) also ended
  `cancelled`, superseded by the newer run of the same pull request.
- No `workflow_dispatch` run occurred in the window; no run of another event
  was cancelled.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the workflow at the head declares types `[opened, reopened, ready_for_review]`, `paths-ignore` `'**.md'`, `'docs/**'`, `'situation/**'`, `'LICENSE*'`, unfiltered `workflow_dispatch`, the same-repository guard, and `runs-on: {group: ci, labels: [automation-test-s]}`. |
| P2 | PASS — the job declares "Required native capabilities" before Formatting, Clippy, Tests and Dependency policy. |
| P3 | PASS — the workflow-level block at the head is exactly D-000020's. |
| P4 | PASS — run 36865937494 (first reopen) `cancelled`; run 36865956045 (second reopen) ran all five steps successfully on `automation-test-s`. |
| P5 | PASS — no run for push 1 `3395760…`; the only runs for push 2 `aeba47e…` are the two reopen runs. |
