# C-000005 — Publish to crates.io

## State

proposed

## Candidate

Publish the crate under the `oidc-client` package name on crates.io once its
release prerequisites and evidence are complete.

## Origin

The standalone-crate decision D-000001 and the name-availability observation
in R-000001 establish this as a possible next step.

## Why consider it

Publication makes the maintained continuation consumable as an ordinary Cargo
dependency rather than a source import.

## Qualification questions

- Is a release credential and publication authority available?
- Do the package metadata and documentation satisfy the release boundary?
- Are P-000001, P-000002, and the CI route sufficiently evidenced for the
  selected release scope?

## Candidate approaches

- Publish `oidc-client` after the applicable promises and CI route are
  qualified.
- Keep the package unpublished while resolving missing evidence.

## Disposition

none — P-000003/O-000003 describe the possible publication behavior, but this
candidate has not been promoted or rejected.
