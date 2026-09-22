# G-000001 — No maintained published Rust OIDC RP crate is established for ES256

## State

open

## Gap

No maintained, published Rust OIDC relying-party crate is established by the
retained research as supporting ES256 (ECDSA P-256 with SHA-256) for ID-token
verification. The `openidconnect` 4.0.1 donor does implement ES256
verification, but its project is recorded as unmaintained; this repository's
continuation is not yet published or assured.

## Relevance

This gap explains why a maintained continuation and its publication matter.
P-000002 records the carried ES256 implementation; P-000003 records the
separate publication commitment. Poda Chat's recorded deployment still uses
RS256 under Kanidm's legacy-crypto setting.

## Evidence

- The retained Poda Chat scoping research in `situation/context.md` reports
  that no maintained alternative was available and that Kanidm 1.11.0 labels
  RS256 as legacy.
- `src/core/jwk/mod.rs` dispatches
  `CoreJwsSigningAlgorithm::EcdsaP256Sha256` to EC signature verification,
  and `src/core/crypto.rs` verifies P-256 signatures with `p256`; the
  imported donor therefore contradicts the earlier claim that openidconnect
  dropped ES256 in the 2.x→4.x rewrite.
- `situation/references/D-000004/R-000001-upstream-pin.md` identifies that donor as
  openidconnect 4.0.1 and records its unmaintained status in the repository
  context.
- Corrector observation in closure run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`: P-000002
  later became assured by W-000003, but P-000003 does not yet assure a
  maintained released package.

## Impact

Until an ES256-capable continuation is both published and assured, consumers
cannot rely on a maintained, released Rust OIDC RP package for an
ES256-defaulting provider. They may remain on an issuer's legacy algorithm or
take on verification work outside the package boundary.

## Resolution

none — P-000002 later assured the ES256 behavior under W-000003, but P-000003
does not yet assure a maintained released package; the current release-bound
absence remains unresolved.

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/promises/P-000003-published-on-crates-io-as-oidc-client.md`
- situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item G-000001
(project Status: Todo). The Evidence section draws on the bootstrap
situation/context.md "Origin evidence" (in-repo at b197618) beyond the
project item body.

