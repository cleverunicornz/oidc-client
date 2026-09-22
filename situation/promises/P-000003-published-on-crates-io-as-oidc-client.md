# P-000003 — Publish oidc-client on crates.io

## State

hypothesis

## Promise

The crate is published on crates.io as `oidc-client` at a semantic version.
Its crates.io page exposes the package documentation, repository URL, and MIT
license, and its CI route runs the repository's formatting, lint, test, and
dependency-policy commands under the root organization workflow policy.

## Scope

Crates.io publication, published-package metadata, and the repository CI route.
The imported OIDC behavior and ES256 behavior remain P-000001 and P-000002;
consumer migration belongs outside this repository.

## Oracle

situation/oracles/O-000003-judge-crates-io-publication.md

## State evidence

None — state is `hypothesis`; no publication or CI workflow-run witness exists.
The prior crates.io name-availability observation in R-000001 is feasibility
evidence for a later qualification, not implementation or assurance.

## Residual

This promise does not assure that a downstream consumer has adopted the crate,
that every possible registry mirror resolves it, or that a future CI run will
remain green after its recorded observation.

## References

- situation/references/R-000001-upstream-pin.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
