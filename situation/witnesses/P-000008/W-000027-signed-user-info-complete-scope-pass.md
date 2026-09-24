# W-000027 — Signed-UserInfo complete-scope observation passes

## Promise

situation/promises/P-000008-signed-user-info-verification-policy.md

## Oracle

situation/oracles/O-000018-judge-signed-user-info-verification-policy-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The retained
`user_info_signed_response_` battery now executes the ES256, ES384, HS256,
HS384, and HS512 fixtures (the ES384/HS384/HS512 legs W-000020 left INVALID are
fixture-executed, not manual), the `user_info_request` battery executes the
request-builder policy and the Client confidential-routing fixture, and the
claim-preservation fixture executes separately. The remaining structural legs
(P1 default allowlist, P7 public-verifier rejection routing, P10/F6
API-surface absence) are decided by code inspection named per leg below.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000027/cargo-test-user-info-signed-response.log`
  retains `cargo test --offline --lib --quiet -- user_info_signed_response_`
  (`exit=0`): `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 105
  filtered out`. SHA-256
  `349b0367ff72d465fa43b9311d9d86dffe01a58f72fa0801636bd518e51e3653`.
- `situation/witnesses/evidence/W-000027/cargo-test-user-info-request.log`
  retains `cargo test --offline --lib --quiet -- user_info_request`
  (`exit=0`): `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 104
  filtered out`. SHA-256
  `f79d975877dbeb627e0cb57ac4601910f4549c4a809496b959088b9f53d0435b`.
- `situation/witnesses/evidence/W-000027/cargo-test-user-info-verified-claims.log`
  retains `cargo test --offline --lib --quiet -- user_info_verified_claims`
  (`exit=0`): `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 109
  filtered out`. SHA-256
  `f4dc5b9691117261d96366c090b31dd4eeeca51c96f809c2aabef3f748a5e62b`.
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
| P1 | PASS — executed: the ES256 fixture asserts the default verifier rejects ES256 with `DisallowedAlg` (`src/verification/tests.rs::test_user_info_signed_response_es256`, lines 1584-1596) and the request-policy fixture asserts the RS256-only default rejects an allowed-secret HS256 build (`src/user_info.rs::test_user_info_request_signed_response_policy`, lines 754-765, 788-791). Structural: `JwtClaimsVerifier::new` seeds `allowed_algs: Some([rsa_sha_256()])` (`src/verification/mod.rs` lines 136-143); both `UserInfoVerifier` constructors (lines 916-927, 935-948) and the `user_info_impl` builder route (lines 102-117 of `src/user_info.rs`) construct through it, so the default is RS256-only on the direct and request-builder surfaces alike. |
| P2 | PASS — `test_user_info_signed_response_es256` verifies an ES256 response against the matching P-256 JWK once explicitly allowed (lines 1571-1582). |
| P3 | PASS — `test_user_info_signed_response_es384` verifies an ES384 response against a matching P-384 JWK once explicitly allowed (lines 1994-2006); the dispatch is the `EcdsaP384Sha384` branch requiring `crv` P-384 in `CoreJsonWebKey::verify_signature` (`src/core/jwk/mod.rs` lines 350-358) behind the single allowlist check (`src/verification/mod.rs` lines 395-417). |
| P4 | PASS — direct: `test_user_info_signed_response_hs256` verifies a confidential HS256 response with an empty JWKS (lines 1667-1682); request-builder: `test_user_info_request_signed_response_policy` verifies an HS256 response through `set_allowed_algs` + `set_client_secret` (lines 727-736). |
| P5 | PASS — direct: `test_user_info_signed_response_hs384` runs the shared HS policy helper, whose allowed-confidential case verifies with an empty JWKS (`src/verification/tests.rs` lines 2088-2100, fixture at 2133-2134); request-builder: the HS384/HS512 loop in `test_user_info_request_signed_response_policy` verifies with the supplied secret (lines 770-792). The `HmacSha384` branch consumes `self.k` (`src/core/jwk/mod.rs` lines 311-325). |
| P6 | PASS — direct: `test_user_info_signed_response_hs512` runs the same helper for HS512 (lines 2141-2142); request-builder: the same loop's HS512 iteration (lines 770-792). The `HmacSha512` branch is `src/core/jwk/mod.rs` lines 326-340. |
| P7 | PASS — executed per algorithm: the HS256 fixture (lines 1698-1708) and the shared helper used by HS384/HS512 (lines 2118-2126) reject each HS* response through a public verifier with `DisallowedAlg` even when explicitly allowed. Structural: `uses_shared_secret()` routes every HS* algorithm to the client-secret branch of `JwtClaimsVerifier::verified_claims` (`src/verification/mod.rs` lines 423-443) and `JwtClaimsVerifier::verification_key` (lines 497-513); with `client_secret: None` both return `DisallowedAlg` errors, with no per-algorithm divergence. |
| P8 | PASS — executed: `test_user_info_request_confidential_client_routes_secret` (`src/user_info.rs` lines 1101-1179) issues `Client::user_info` on a confidential client with an empty JWKS and verifies the HS256 response without further configuration (lines 1145-1160), proving the secret reaches the signed-response verifier. Structural: `user_info_impl` selects `UserInfoVerifier::new_confidential_client` exactly when `self.client_secret()` is `Some` (`src/user_info.rs` lines 102-117). |
| P9 | PASS — `cargo test --offline --lib --quiet -- user_info_verified_claims` executes `test_user_info_verified_claims`, which enforces issuer, audience, and expected-subject checks on verified signed claims. The enforcement points are the issuer check (`src/verification/mod.rs` lines 320-333), audience check (lines 340-373), and expected-subject check (lines 1009-1022). |
| P10 | PASS — structural inspection of the public surfaces at this head: `UserInfoVerifier`'s inherent methods (`src/verification/mod.rs` lines 908-1024) are `new`, `new_confidential_client`, `require_issuer_match`, `require_audience_match`, `set_allowed_algs`, `set_client_secret` (plus `pub(crate)` accessors) — no allow-any method; `UserInfoRequest`'s inherent methods (`src/user_info.rs` lines 136-327) end in `set_allowed_algs` (299), `set_client_secret` (315), `set_response_type` (323) — no allow-any method. `JwtClaimsVerifier::allow_any_alg` (lines 189-192) is reachable only through the `pub(crate)` struct (line 117). The public `IdTokenVerifier::allow_any_alg` (lines 630-633) belongs to the ID-token surface, which the Promise's Scope excludes. |
| F1 | PASS (negative executed) — default/non-allowlisted acceptance is rejected per algorithm: ES256 default rejection (lines 1584-1596), ES384 default rejection (lines 2008-2020), HS384/HS512 default-with-secret rejection via the builder loop (lines 788-791), HS256 default-with-secret rejection via the builder (lines 754-765). |
| F2 | PASS (negative executed) — altered-signature and wrong-key ES256 rejections (lines 1598-1627); altered-signature and wrong-key ES384 rejections (lines 2022-2051). |
| F3 | PASS (negative executed and structural) — wrong-secret HS256 direct rejection (lines 1684-1696); missing-secret builder rejection (lines 739-750); wrong-secret HS384/HS512 direct rejection via the helper (lines 2103-2114); missing-secret HS384/HS512 builder rejection (lines 784-786). Structural: the builder stores whichever secret `set_client_secret` supplies (`src/user_info.rs` lines 315-320) and `JwtClaimsVerifier::verification_key` derives the symmetric key from the held secret alone (`src/verification/mod.rs` lines 502-507), so a wrong secret on the builder path fails HMAC verification exactly as the executed direct-verifier wrong-secret cases do. |
| F4 | PASS (negative executed) — the same confidential-routing fixture rejects the identical response through a public client with `DisallowedAlg` after both requests dispatched (`src/user_info.rs` lines 1162-1178, `dispatched.len() == 2` at 1178), so the failure is verification, not lost routing. Structural: the `new_confidential_client` selection at `src/user_info.rs` lines 102-117 preserves the secret in the verifier (`src/verification/mod.rs` lines 942-944). |
| F5 | PASS (negative executed) — mismatched issuer/audience/expected-subject rejections execute in `test_user_info_verified_claims` and in `test_user_info_subject_binding` (`src/verification/tests.rs` lines 1763-1774). |
| F6 | PASS (negative structural) — same inspection as P10: no UserInfo API can disable the allowlist or accept any algorithm; the only `allow_any_alg` definitions in the tree are `pub(crate)` (`src/verification/mod.rs` line 189) and on the out-of-scope ID-token verifier (line 630). |
