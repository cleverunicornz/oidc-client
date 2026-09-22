# P-000002 — ES256 (ECDSA P-256) ID-token verification

## State

implemented

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

State `implemented` is retrospective and is supported by the donor import
commit `eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a`: its source contains the
ES256 dispatch in `src/core/jwk/mod.rs` and P-256 verifier in
`src/core/crypto.rs`. The retained gate observation at
`24835e4b44caa8a0baae2ac5b865bbd6bdf355ba` is captured as the retrospective,
`INVALID` partial witness
`situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md`.
`situation/decisions/D-000006-recognize-carried-es256-verification.md`
reconciles the earlier hypothesis against those facts. The state is not
`assured`: no complete PASS witness applies O-000002 to every leg.

## Residual

No retained witness yet exercises a complete ES256 ID-token validation through
a real provider or a full ID-token fixture, so signature plus issuer,
audience, nonce, and expiry validation remain unassured (G-000006). ES384 and
ES512 are outside this promise. Mismatched-curve behavior is not an in-scope
claim.

## References

- situation/decisions/D-000006-recognize-carried-es256-verification.md
- situation/gaps/G-000001-no-maintained-rust-oidc-rp-library-supports-es256.md
- situation/gaps/G-000006-no-full-es256-id-token-fixture.md
- situation/witnesses/evidence/W-000001/gates-final.log
- situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md
