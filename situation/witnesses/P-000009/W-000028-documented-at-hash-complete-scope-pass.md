# W-000028 — Documented at_hash complete-scope observation passes

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000019-judge-documented-at-hash-flow-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The retained
`id_token_verification_key_at_hash` battery now executes three fixtures: the
HS256/ES256 representative (including the missing-secret and
substituted-token failures), an HS384/HS512 fixture, and an ES384 fixture, so
the algorithm legs W-000021 left INVALID are fixture-executed. The
documentation leg executes the crate-root tutorials with `reqwest-blocking`
enabled. The `signing_key` scope leg is decided by code inspection named
below.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000028/cargo-test-id-token-verification-key-at-hash.log`
  retains `cargo test --offline --lib --quiet -- id_token_verification_key_at_hash`
  (`exit=0`): `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 107
  filtered out`. The selected tests are
  `test_id_token_verification_key_at_hash`,
  `test_id_token_verification_key_at_hash_hs384_hs512`, and
  `test_id_token_verification_key_at_hash_es384`. SHA-256
  `a7fe0ff9925816e7c5f126a70dfe87ab418f43f0bc143c0aedd714267b957e24`.
- `situation/witnesses/evidence/W-000028/cargo-test-doc-reqwest-blocking.log`
  retains `cargo test --offline --doc --features reqwest-blocking` (`exit=0`):
  `test result: ok. 7 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out`.
  The compiled crate-root tutorials are the doctests at `src/lib.rs` lines 134
  (`Getting started: Authorization Code Grant w/ PKCE`), 285, 377, 418, and
  510 (`Asynchronous API`), plus `src/client.rs` line 33 and
  `src/jwt/mod.rs` line 61; the two ignored doctests are the `rust,ignore`
  signature sketches at `src/lib.rs` lines 87 and 95. SHA-256
  `64a866971a6616f5f5f7429ead1d8005449d9f53a923b19b838fdcda975b64fc`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_id_token_verification_key_at_hash` verifies an HS256 ID token through a confidential verifier with an empty JWKS and reproduces the access-token hash through `verification_key` followed by `AccessTokenHash::from_token` (`src/verification/tests.rs` lines 1825-1849). |
| P2 | PASS — fixture-executed (superseding the oracle's manual route): `test_id_token_verification_key_at_hash_hs384_hs512` verifies HS384/HS512 ID tokens through a confidential verifier with an empty JWKS and reproduces each access-token hash from the client-secret-derived key (fixture at `src/verification/tests.rs` lines 2210-2288). The hash resolution is the `HmacSha384`/`HmacSha512` arms of `CoreJsonWebKey::hash_bytes` (`src/core/jwk/mod.rs` lines 393-408). |
| P3 | PASS — same fixture, HS512 arm (lines 2210-2288; hash arm `src/core/jwk/mod.rs` lines 401-408). |
| P4 | PASS — the ES256 arm of the main fixture resolves the matching provider JWK through `verification_key`, proves the resolved key hashes exactly like the fixture key, and reproduces the access-token hash (`src/verification/tests.rs` lines 1873-1938). |
| P5 | PASS — fixture-executed (superseding the oracle's manual route): `test_id_token_verification_key_at_hash_es384` verifies an ES384 ID token against a matching P-384 provider JWK and reproduces its access-token hash (`src/verification/tests.rs` lines 2292 onward; SHA-384 hash arm `src/core/jwk/mod.rs` lines 393-400). |
| P6 | PASS (negative executed) — the shared-secret token used with a public verifier holding no client secret returns `Err(SignatureVerificationError::DisallowedAlg)`, not a panic (`src/verification/tests.rs` lines 1860-1871; enforcement at `src/verification/mod.rs` lines 502-511). |
| P7 | PASS (negative executed) — a substituted access token produces a different hash: `assert_ne!` on `AccessTokenHash::from_token` over `"substituted_access_token"` (`src/verification/tests.rs` lines 1851-1858). |
| P8 | PASS — structural: `IdToken::signing_key` (`src/id_token/mod.rs` lines 169-179) borrows the verifier (`&'s IdTokenVerifier<'s, K>`) and returns `Result<&'s K, SignatureVerificationError>`; it delegates to `JwtClaimsVerifier::signing_key` (`src/verification/mod.rs` lines 459-491), which resolves keys exclusively from `self.signature_keys.filter_keys(...)` with no client-secret branch. The documented JWKS-only borrowed scope (doc comment lines 159-165) is intact at this head. |
| P9 | PASS — `cargo test --offline --doc --features reqwest-blocking` passes 7 doctests including both crate-root tutorials that chain `id_token.verification_key(&id_token_verifier)?` into `AccessTokenHash::from_token` (`src/lib.rs` lines 229-234 in the PKCE tutorial, lines 607-612 in the Asynchronous API tutorial); both compile as `rust,no_run` doctests (`- compile ... ok`) with the feature enabled. |
| F1 | PASS (negative executed and structural) — HS256 rejects through the public verifier with `Err(DisallowedAlg)`, not a panic (lines 1860-1871). The hs384_hs512 fixture constructs only confidential verifiers (lines 2210-2288), so the HS384/HS512 public-verifier rejection is decided structurally: `uses_shared_secret()` routes every HS* algorithm to the same client-secret branch of `verification_key` (`src/verification/mod.rs` lines 502-511) with no per-algorithm divergence, and a verifier holding no secret takes the `DisallowedAlg` error arm (lines 508-510). The confidential flows never require a provider JWKS key (empty JWKS in all three confidential cases). |
| F2 | PASS (negative executed and structural) — the ES256 arm resolves the matching provider JWK and reproduces the expected access-token hash (lines 1917-1938: resolved-key and fixture-key hashes agree and `AccessTokenHash::from_token` reproduces the `at_hash`), and the ES384 fixture executes the same resolution for P-384. Structural: `hash_bytes` hashes only the supplied token bytes after an algorithm-compatibility check (`src/core/jwk/mod.rs` lines 381-400), so hash reproduction alone does not distinguish keys; the negative is carried by the same documented flow's signature verification, which both fixtures run first (`claims(...)` at `src/verification/tests.rs` lines 1836-1838 and 1911-1913; `jwt.payload` at `src/verification/mod.rs` line 453), and whose key selection filters by key ID and algorithm compatibility (`signing_key`, `src/verification/mod.rs` lines 464-469; EC curve checks `src/core/jwk/mod.rs` lines 350-358). A nonmatching provider JWK fails resolution there rather than yielding a usable verification key; no fixture executes a nonmatching-key rejection on the at_hash surface itself. |
| F3 | PASS (negative executed) — the substituted-token `assert_ne!` (lines 1851-1858). |
| F4 | PASS — the same doctest battery compiles both tutorials against the owned-key `verification_key` call (P9 evidence). |
| F5 | PASS — structural, same inspection as P8: the borrowed JWKS-only scope of `signing_key` is unchanged at this head. |
