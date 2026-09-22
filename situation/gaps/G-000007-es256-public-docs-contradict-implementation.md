# G-000007 — ES256 public documentation contradicts implementation

## State

open

## Gap

The public `CoreJwsSigningAlgorithm::EcdsaP256Sha256` documentation says
"currently unsupported", while the carried source dispatches that enum variant
to P-256 verification and P-000002 records an implemented ES256 path.

## Relevance

Consumer-facing documentation and repository orientation must not make a
stronger or weaker ES256 claim than the recorded implementation and assurance
state. This conflict is distinct from the missing full-ID-token fixture in
G-000006.

## Evidence

- `src/core/mod.rs` lines 634–636 label the ES256 enum variant "currently
  unsupported".
- `src/core/jwk/mod.rs` lines 308–315 dispatch the same variant with a P-256
  key to `crypto::verify_ec_signature`.
- `src/core/crypto.rs` lines 99–145 construct and use a P-256 verifying key.
- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md`
  records the evidence-bound claim decision.

## Impact

A consumer reading generated API documentation could conclude that the ES256
path is unavailable, while an unqualified repository claim could incorrectly
call it first-class. Neither interpretation accurately expresses the current
implemented-but-unassured state.

## Resolution

none

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md`
