# G-000036 — W-000041 states the `build` label time rounded to the minute and its run-count read as "45 s later"

## State

open

## Gap

- [W-000041](situation/witnesses/P-000018/W-000041-ci-runs-on-every-push.md) leg L4 says: "After the `build` label (02:42Z), the head still had exactly 1 run (`total_count` 1,
  read 45 s later)". Neither time is exact.
  - The label time is rounded: GitHub's issue timeline of pull request #10 records the `build` label at
    **02:41:47Z**, not 02:42Z.
  - "45 s" is the length of the wait loop that preceded the read, and that loop began after all three repositories'
    label calls. It is not the interval from this label to the read.
  - The read itself happened after the poda-public gate run whose trailer reads 02:42:50Z, and before this repository's gate run, whose trailer (`date -u`, written
    after the gate returned) reads **02:42:56Z**.
- The witness is merged and immutable; this Gap retains the exact times.

## Relevance

[W-000041](situation/witnesses/P-000018/W-000041-ci-runs-on-every-push.md) leg L4 of [P-000018](situation/promises/P-000018-ci-runs-on-every-push.md); the build trigger model
rollout.

## Evidence

- Observation: `gh api repos/cleverunicornz/oidc-client/issues/10/timeline` → `labeled` `build` at 02:41:47Z.
- Observation: the retained gate output `trigger-rollout-evidence/oidc-client-gate-S1-head.txt` (orchestration handoff, not in this
  repository) ends with the trailer 02:42:56Z.
- Observation: a witness-time audit of 2026-10-02 (orchestration handoff, not in this repository) found the rounding.
- Interpretation: the read came later after the label than "45 s" says. A later read with still exactly one run is
  at least as strong for L4 (the label started no run), so the PASS does not change on these times.

## Impact

A reader taking 02:42Z or "45 s" as exact gets the label instant and the read interval wrong; the leg's result is
unaffected.

## Resolution

none
