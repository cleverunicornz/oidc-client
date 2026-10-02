# W-000041 — CI runs on every push on pull request #10

## Promise

[P-000018](situation/promises/P-000018-ci-runs-on-every-push.md)

## Oracle

[O-000028](situation/oracles/O-000028-ci-runs-on-every-push.md)

## Result

PASS

## Head

`cd00ad324b6f14d1047e2230f14c56713a17b2a8`

## Observed

2026-10-02

## Evidence

Pull request [#10](https://github.com/cleverunicornz/oidc-client/pull/10), not a draft. Timeline: opened at 02:20:2xZ with `6c055806`; S1 `cd00ad32` (an empty commit) pushed 02:20:45Z, once the opened run's job existed; `build` labeled after S1.

Read with `actions_read list_runs`, `gh api repos/cleverunicornz/oidc-client/actions/runs?head_sha=<head>`,
`.../runs/<id>/attempts/<n>/jobs`, `.../actions/jobs/<id>` (steps) and `.../check-runs/<job id>/annotations`. Every run
has event `pull_request` and attempt 1.

| Run | Head | Result |
|---|---|---|
| [36955165100](https://github.com/cleverunicornz/oidc-client/actions/runs/36955165100) | `6c055806` | created 02:20:25Z, job 02:20:34–02:21:14Z: **cancelled**; job 110676421002 annotation "Canceling since a higher priority waiting request for CI-10 exists" |
| [36955196628](https://github.com/cleverunicornz/oidc-client/actions/runs/36955196628) | `cd00ad32` | job 02:21:25–02:23:28Z: **success** |

The retained listings and gate output are in the orchestration handoff (`trigger-rollout-evidence/oidc-client-*`), not in
this repository; every figure above is re-derivable from the GitHub API calls named.

## Oracle legs

| Leg | Evidence |
|---|---|
| L1 | infra-v2 `scripts/check-workflow-concurrency.py` at `0f6aa5cc` (sha256 `ae69546e…`) on the head: `ok ci.yml (pull_request, light)`; on the base it failed only on the missing `synchronize`. |
| L2 | The S1 head's run [36955196628](https://github.com/cleverunicornz/oidc-client/actions/runs/36955196628) executed the job to `success`. |
| L3 | The run of the previous push, in progress when S1 arrived, ended `cancelled` with the supersession annotation naming `CI-10` (table). |
| L4 | After the `build` label (02:42Z), the head still had exactly 1 run (`total_count` 1, read 45 s later); nothing was cancelled. |
| L5 | infra-v2 `scripts/heavy-build-green.sh cleverunicornz/oidc-client 10` (pinned `0f6aa5cc`) at 02:42:56Z: exit 0, `green ci.yml (light)`, "GREEN: every applicable workflow is green on the exact head". |

No dispatch was run. The head after this Witness gets its own run by the push that adds it (`synchronize`).
