# P-000002 — ES256 (ECDSA P-256) ID-token verification

## State

assured

## Promise

The crate verifies an ES256 (ECDSA P-256 with SHA-256) JWS signature used by
an ID token when the matching EC P-256 JWK is supplied. It accepts that JWK
during key selection, parses `ES256` in provider discovery metadata, and
applies issuer, audience, nonce, and expiry validation through the public
ID-token verifier for an ES256-signed ID token.

## Scope

The carried `CoreJsonWebKey` and generic public ID-token verification path for
ES256 with EC P-256 public keys: signature verification, key selection,
discovery-metadata parsing, and issuer/audience/nonce/expiry validation. This
promise does not promise a provider deployment, publication, ES384 or ES512,
or mismatched-curve behavior.

## Oracle

situation/oracles/O-000002-judge-es256-verification.md

## State evidence

State `assured` cites Oracle
situation/oracles/O-000002-judge-es256-verification.md and the complete PASS
witness
situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md,
observed 2026-09-22 at head
`52e84d5dff27ac026ac7670370e3dda918cbeaf7` on `bank2/assurance`: a
pinned-container `cargo test --all-features` run that passes every oracle
leg, including the previously manual P4 leg through the public ID-token
verifier. The implementation origin remains the donor import commit
`eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a` (ES256 dispatch in
`src/core/jwk/mod.rs`, P-256 verifier in `src/core/crypto.rs`), reconciled by
situation/decisions/D-000006-recognize-carried-es256-verification.md. The
retained
situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md
remains the historical, `INVALID` partial observation it was at
`24835e4b44caa8a0baae2ac5b865bbd6bdf355ba`.

## Residual

ES384 and ES512 are outside this promise. Mismatched-curve behavior is not
an in-scope claim. This promise does not promise a provider deployment or
publication.

## References

- situation/decisions/D-000006-recognize-carried-es256-verification.md
- situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md
- situation/gaps/G-000006-no-full-es256-id-token-fixture.md
- situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md
- situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md
