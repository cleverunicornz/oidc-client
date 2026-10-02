# O-000028 — CI runs on every push, judged from the file, GitHub run data and the gate

## State

designed

## Judges

[P-000018](situation/promises/P-000018-ci-runs-on-every-push.md)

## Inputs

- `.github/workflows/ci.yml` at the judged head.
- infra-v2's `scripts/check-workflow-concurrency.py`, `scripts/heavy_build_green.py` and `scripts/heavy-build-green.sh`
  pinned at `0f6aa5cc` (`Private: cleverunicornz/infra-v2@0f6aa5cc#scripts/`; organization access), run from a console
  with `gh`.
- The runs of the implementing pull request (non-draft) and their jobs and annotations (`actions_read list_runs`,
  `gh api repos/cleverunicornz/oidc-client/actions/runs/<id>/jobs`, check-run annotations):
  - S1: a push of a new head while the run of the previous event has a job;
  - S2: the `build` label added.
- The gate run against the pull request after S1's run concludes.

## Pass

- **L1 — shape.** The checker prints `ok` for `ci.yml` (light: `synchronize` in its types, D-000203's block).
- **L2 — every push (S1).** The S1 head has a `pull_request` run that executed the job to `success`.
- **L3 — supersession.** The previous event's run, queued or in progress at S1, ended `cancelled` with the annotation
  "Canceling since a higher priority waiting request for `<workflow>-<number>` exists".
- **L4 — no heavy work (S2).** The `build` label starts no run of a heavy workflow (the repository has none) and
  cancels nothing.
- **L5 — the gate.** The gate exits 0 once S1's run is green on the same head.

## Fail

- Any leg does not hold; or a push starts no run; or a run is cancelled by anything but a newer run of the same pull
  request.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| L1 | shape | manual — infra-v2 checker run by hand (pinned) |
| L2 | a push runs the job | manual — runs and jobs API |
| L3 | the older run is superseded | manual — runs, jobs, annotations API |
| L4 | the label starts and cancels nothing | manual — runs API |
| L5 | gate green on the exact head | manual — infra-v2 gate run by hand (pinned) |
