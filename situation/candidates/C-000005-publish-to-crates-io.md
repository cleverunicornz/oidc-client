# C-000005 — Publish to crates.io

## State

promoted

## Candidate

Publish the crate under the `oidc-client` package name on crates.io once its
release prerequisites and evidence are complete.

## Origin

The standalone-crate decision D-000001 and the name-availability observation
in `situation/references/D-000004/R-000001-upstream-pin.md` establish the
selected publication direction.

## Why consider it

Publication makes the maintained continuation consumable as an ordinary Cargo
dependency rather than a source import.

## Qualification questions

- Is a release credential and publication authority available?
- Do the package metadata and documentation satisfy the release boundary?
- Are P-000001, P-000002, and the selected release gates sufficiently
  evidenced for the release scope?

These remain P-000003 implementation and assurance questions after selection.

## Candidate approaches

- Publish `oidc-client` after the applicable promises and CI route are
  qualified.
- Keep the package unpublished while resolving missing evidence.

## Disposition

Promoted by
`situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md`
into P-000003 and O-000003. P-000003 remains a hypothesis pending feasibility
evidence and publication.
