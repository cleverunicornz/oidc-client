# D-000013 — Pre-publication hardening maintenance deltas for the UserInfo surface

## Status

accepted

## Date

2026-09-23

## Context

The pre-publication review of the 4.1.0 continuation line identified three
defects on the user info / ID-token surface: the documented user info flows
did not bind the verified ID-token subject (OIDC Core 5.3.2); the signed
UserInfo verifier was hardwired to the RS256 default with no way to configure
algorithms or confidential credentials; and the documented `at_hash` flow
resolved verification keys from the provider JWKS only, so it failed for
HS256/384/512 ID tokens whose key is the client secret. Fixing them changes
donor-derived bytes, which D-000004's import-fidelity regime classifies as
defects unless authorized.

## Evidence

- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the import-fidelity regime this decision scopes itself against.
- situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md and
  situation/promises/P-000007-hmac-sha2-and-es384-jws-verification.md — the
  assured verification behavior this work must not regress; the scoped runs of
  `src/verification/tests.rs::test_es256_id_token_verified_claims` and
  `src/core/jwk/tests.rs::test_hmac_sha256_verification` /
  `::test_ecdsa_verification` passed after the change.
- The Stream A focused tests:
  `src/verification/tests.rs::test_user_info_signed_response_es256`,
  `::test_user_info_signed_response_hs256`, `::test_user_info_subject_binding`,
  `::test_id_token_verification_key_at_hash`, and
  `src/user_info.rs::test_user_info_request_signed_response_policy`.
- The pre-publication review findings directed by the parent session
  (branch `fix/prepublication-hardening`).

## Decision

1. The following byte deltas are deliberate maintenance deltas of the
   continued crate, classified as `maintenance-feature` (new public behavior
   on an owned continuation line), superseding D-000004's regime for exactly
   this surface and no other: `src/user_info.rs` (confidential verifier
   routing in `user_info_impl`; `UserInfoRequest::set_allowed_algs` and
   `UserInfoRequest::set_client_secret` builders; test module addition),
   `src/verification/mod.rs` (`UserInfoVerifier::new_confidential_client`,
   `::set_allowed_algs`, `::set_client_secret`, expanded docs; owned key
   resolution on the private claims verifier), `src/verification/tests.rs`
   (four new tests plus import changes), `src/id_token/mod.rs`
   (`IdToken::verification_key`; `signing_key` scope documentation),
   `src/lib.rs` (crate-root tutorial migrations only: `verification_key` call,
   subject-bound `user_info` call, OIDC Core 5.3.2 note), `examples/gitlab.rs`
   (subject-bound `user_info` call), and one accessor
   (`Client::client_secret`, `pub(crate)`) in `src/client.rs` authorized by
   the parent session. D-000004 itself remains unchanged and authoritative for
   all other imported bytes.
2. `UserInfoVerifier` and `UserInfoRequest` expose no allow-any-algorithm
   method. The relying party did not request the user info JWT's algorithm the
   way it requests the ID-token algorithm during registration, so an explicit
   allowlist is the only supported configuration and defends against algorithm
   confusion. (`IdTokenVerifier::allow_any_alg` is untouched.)
3. The `at_hash` API shape is an owned-key resolution method:
   `IdToken::verification_key(&self, verifier: &IdTokenVerifier<'_, K>) ->
   Result<K, SignatureVerificationError>`, which derives the symmetric key via
   `K::new_symmetric` from the client secret when the token's algorithm
   `uses_shared_secret()` and a secret is present, and otherwise resolves from
   the verifier's key set. Missing secret is an error, never a panic.

## Why

A continuation line owns its donors' defects; the maintenance-delta
classification keeps the delta table auditable without freezing the API.
Excluding allow-any on the UserInfo surface closes the last unconstrained
algorithm path in the documented flows. The owned-key method keeps the
existing borrowed, JWKS-only `signing_key` contract intact for its existing
callers while giving the tutorials one uniform call for both key kinds.

## Rejected alternatives

- Extending `IdToken::signing_key` to derive symmetric keys: would silently
  change a borrowed-return contract and conflate JWKS resolution with
  credential derivation for existing callers.
- A verifier-side hash helper (computing the `at_hash` comparison inside the
  verifier): duplicates `AccessTokenHash::from_token` and hides the
  hash-comparison step the tutorials teach consumers to perform explicitly.
- Exposing `allow_any_alg` on `UserInfoVerifier`/`UserInfoRequest` for parity
  with `IdTokenVerifier`: rejected per Decision §2 — no legitimate consumer
  flow requests an unconstrained UserInfo algorithm.
- Applying the client's ID-token algorithm registration
  (`id_token_signing_algs`) to the signed-UserInfo verifier: would change the
  default for existing users; the signed-UserInfo allowlist is configured
  explicitly.

## Consequences

Future diffs against the donor pin over this surface are judged against this
decision (maintenance deltas) rather than flagged as fidelity defects. The
default signed-UserInfo allowlist remains RS256-only for both public and
confidential clients; consumers opt in per algorithm. New public API:
`UserInfoVerifier::new_confidential_client`, `UserInfoVerifier::set_allowed_algs`,
`UserInfoVerifier::set_client_secret`, `UserInfoRequest::set_allowed_algs`,
`UserInfoRequest::set_client_secret`, `IdToken::verification_key`.
`Client::user_info`'s signature is unchanged.

## Revisit when

An upstream re-sync touches this surface (re-derive the delta table), or a
consumer flow legitimately requires server-signed UserInfo verification
without registration-time algorithm agreement.

## Provenance

Recorded 2026-09-23 by Stream A (UserInfoPolicy) of the pre-publication
hardening batch; the `src/client.rs` accessor was authorized mid-flight by the
parent session and is disclosed here and in the stream's report.
