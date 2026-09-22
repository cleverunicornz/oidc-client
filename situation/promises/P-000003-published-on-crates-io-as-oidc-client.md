# P-000003 — Publish oidc-client on crates.io

## State

hypothesis

## Promise

The crate is published on crates.io as `oidc-client` at a semantic version.
Its crates.io page exposes the package documentation, repository URL, and MIT
license.

## Scope

Crates.io publication and published-package metadata. The imported OIDC
behavior and ES256 behavior remain P-000001 and P-000002; configured CI is
P-000005 and consumer migration belongs outside this repository.

## Oracle

situation/oracles/O-000003-judge-crates-io-publication.md

## State evidence

None — state is `hypothesis`; no qualifying Witness establishes publication
feasibility. The retained crates.io name-availability fact is a prerequisite
observation, not a qualifying Witness, implementation, or assurance.

## Residual

This promise does not assure downstream adoption, every possible registry-mirror
resolution, or any CI route. P-000005 separately owns configured CI behavior.

## References

- situation/references/D-000004/R-000001-upstream-pin.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
- situation/candidates/C-000005-publish-to-crates-io.md
- situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md
