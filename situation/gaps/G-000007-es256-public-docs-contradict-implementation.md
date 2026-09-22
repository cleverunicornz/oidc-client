# G-000007 — ES256 public documentation contradicts implementation

## State

closed

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
implemented and assured state.

## Resolution

closed — the enum documentation was corrected per variant against the
dispatch code: `HmacSha256`, `HmacSha384`, and `HmacSha512` are verified via
the `hmac` arms (`src/core/jwk/mod.rs` lines 263–300), `EcdsaP256Sha256` and
`EcdsaP384Sha384` are dispatched to `crypto::verify_ec_signature` (P-256 and
P-384 arms of `src/core/crypto.rs`), so those five variants dropped the
false "(currently unsupported)" label. `EcdsaP521Sha512` keeps the label:
it has no verification arm and falls through to
`SignatureVerificationError::UnsupportedAlg`, and no P-521 crate is a
declared dependency. The wording states the implemented surface without
asserting an assurance level, keeping the documentation in agreement with
P-000002's evidence-bound claims (D-000007).

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md` —
  the claims-bounding decision this correction executes.
- `src/core/jwk/mod.rs` and `src/core/crypto.rs` — the dispatch code each
  doc comment was verified against.

## Provenance

Corrected in place on open PR #2 before any closing checkpoint (CodeRabbit
thread PRRT_kwDOUlFWIM6kzMf2): the current-state wording
"implemented-but-unassured" became "implemented and assured" once W-000003
assured P-000002. The Gap's historical observations and resolution are
unchanged.
