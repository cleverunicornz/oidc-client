# G-000004 — No assured CI witness route

## State

closed

## Gap

No local Promise, Oracle, and PASS Witness lineage presently assures a CI gate
claim. The repository has a workflow configuration and an offline local gate
observation, but no retained witness of that workflow executing on the
configured fleet runner.

## Relevance

The root `AGENTS.md` repository block must describe verification as unassured
until a recorded Promise, matching Oracle, and Witness support the gate claim.
This absence also bounds the CI portion of
`situation/promises/P-000003-published-on-crates-io-as-oidc-client.md`.

## Evidence

- `.github/workflows/ci.yml` defines the `ci` job on
  `cvu-test-runner-x64`, but this opening tree retains no workflow-run URL or
  witness for that job.
- `situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md`
  records an `INVALID` partial observation from a local pinned-container gate
  run; it is not a CI workflow witness.
- `situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md`
  remains in qualification pending the workflow's first real fleet run.

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`: the annotation at
  `.github/workflows/ci.yml` line 43 says "70 offline unit + doc tests", while
  the retained `gates-final.log` records 70 unit tests plus 7 passing and 2
  ignored doctests. The executed command is unaffected, but the configured
  route's count annotation is stale.
- Corrector observation in closure run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`: every retained
  CI run now has its own Witness — W-000004 for the primary PASS, W-000007 and
  W-000008 for later PASS runs, and W-000009 and W-000010 for the failed
  dispatch and pull-request observations.

## Impact

The workflow is evidence of an intended gate route, not assurance that a gate
claim has executed or passed. Repository orientation must not advertise a
verified CI gate until a qualifying run is retained as a Witness.

## Resolution

closed — the prescribed assured-witness route is executed: W-000004 retains
the successful dispatched run 35743515109 at
`7fe8265c8166e16ea4da77b5722a35503fa67665`, which executes the capability
checks and all four named gate commands on `cvu-test-runner-x64`. P-000005 is
assured under O-000006 by that bounded PASS witness. W-000007 and W-000008
retain later successful dispatches at their own heads; W-000009 and W-000010
preserve the failed dispatch and pull-request observations rather than
combining them with the passing evidence.

## References

- `.github/workflows/ci.yml`
- `situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md`
- situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md —
  the retained local-run observation (not workflow evidence).
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/witnesses/P-000005/W-000007-configured-ci-public-entry-fixture-run.md
- situation/witnesses/P-000005/W-000008-later-passing-configured-ci-run.md
- situation/witnesses/P-000005/W-000009-configured-ci-dispatch-clippy-failure.md
- situation/witnesses/P-000005/W-000010-configured-ci-pull-request-clippy-failure.md
- situation/gaps/G-000021-ci-test-count-comment-is-stale.md
