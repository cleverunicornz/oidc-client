# O-000002 — Judge ES256 verification

## State

designed

## Judges

situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Inputs

ID tokens from a real OIDC provider signing with ES256 (Kanidm or a test
fixture), the provider's discovery metadata and JWK set, and tampered ES256
tokens.

## Pass

- P1: ES256-signed ID token from a real provider (Kanidm or test fixture)
  verifies: signature, issuer, audience, nonce, expiry
- P2: Elliptic curve JWK (crv=P-256) accepted during key selection
- P3: Discovery metadata with ES256 in id_token_signing_alg_values_supported
  accepted
- P4: Tampered ES256 token rejected

## Fail

- F1: ES256 token from a real provider cannot be verified
- F2: EC JWK rejected during key selection
- F3: Tampered token accepted

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item O-000002
(project Status: Todo). Corrected for contract coherence: the project body
lists "P5: ES384 optionally supported (stretch goal)"; ES384 is outside
P-000002's declared Scope, and a non-gating Pass leg cannot exist under the
witnesses law (every Pass leg must be evidenced). The stretch goal remains
possible future work, named in the promise's Scope.
