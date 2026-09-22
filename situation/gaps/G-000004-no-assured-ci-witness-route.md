# G-000004 — No assured CI witness route

## State

open

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

## Impact

The workflow is evidence of an intended gate route, not assurance that a gate
claim has executed or passed. Repository orientation must not advertise a
verified CI gate until a qualifying run is retained as a Witness.

## Resolution

none

## References

- `.github/workflows/ci.yml`
- `situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md`
- `situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md`
