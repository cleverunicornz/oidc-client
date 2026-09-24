# W-000037 — at_hash complete-scope successor PASS

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md

## Result

PASS — every Pass and Fail leg of O-000025 is decided at this head: the
executable legs by the battery run retained below, the manual legs by the
inspections cited per row. This is the complete-scope successor's own
observation at the current head; the retained W-000035 and W-000028 records
remain their own immutable observations and are not composed into this one.
P-000009's canonical State is frozen by the third closure; Promise-state
transitions and Gap resolutions belong to the fourth closure's corrector and
are not performed here.

## Head

c5260aeb4a426f7881c8261ef2101d5a30710c29

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000037/cargo-test-id-token-verification-key-at-hash.log`
  retains `cargo test --offline --lib --quiet --
  id_token_verification_key_at_hash` (`exit=0`): `test result: ok. 4 passed;
  0 failed; 0 ignored; 0 measured; 108 filtered out; finished in 0.14s`. It
  executes all four named fixtures: `test_id_token_verification_key_at_hash`,
  `test_id_token_verification_key_at_hash_hs384_hs512`,
  `test_id_token_verification_key_at_hash_es384`, and
  `test_id_token_verification_key_at_hash_rsa_pss_eddsa`. SHA-256
  `f7621df30b08f91e012dd6f6531b9a38270ec38b2da8d23fb288106348f79f9b`.
- `situation/witnesses/evidence/W-000037/cargo-test-doc-reqwest-blocking.log`
  retains `cargo test --offline --doc --features reqwest-blocking` (`exit=0`):
  `test result: ok. 7 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out;
  finished in 0.17s`. Both crate-root tutorials compile in this run as
  `test src/lib.rs - (line 134) - compile ... ok` and
  `test src/lib.rs - (line 510) - compile ... ok`. SHA-256
  `f3fee3f1cecc3080cc4082cb12ca50b188c0367f88272d28eaeab5ae606f0fa8`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `c5260aeb4a426f7881c8261ef2101d5a30710c29`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness decides the successor oracle's legs at this head only;
  it performs no Promise-state transition and resolves no Gap.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | Observed — battery: the HS256 ID token verifies through the confidential verifier with an empty JWKS (`src/verification/tests.rs` lines 1829-1839), `verification_key` resolves from the client secret (lines 1841-1843), and `AccessTokenHash::from_token` reproduces the embedded `at_hash` (`assert_eq!` at line 1850). |
| P2 | Observed — battery: `test_id_token_verification_key_at_hash_hs384_hs512`, `HmacSha384` iteration verifies through the confidential verifier with an empty JWKS and reproduces the hash (lines 2286-2333: verifier 2302-2309, claims 2310-2312, `from_token` 2317-2323). |
| P3 | Observed — battery: same fixture, `HmacSha512` iteration (lines 2286-2333). |
| P4 | Observed — battery: `test_id_token_verification_key_at_hash` ES256 leg — claims verify against the matching provider JWK (lines 1912-1914), `verification_key` resolves it (lines 1915-1917), the resolved key hashes identically to the fixture key (lines 1919-1931), and `AccessTokenHash::from_token` reproduces the access token's at_hash: the hand-signed ES256 payload (lines 1892-1893) carries no embedded `at_hash` claim, so the fixture asserts the documented call's output against the spec-defined expected value — the base64url left half of SHA-256 over the access token, computed independently from the fixture key (lines 1932-1939). |
| P5 | Observed — battery: `test_id_token_verification_key_at_hash_es384` with the matching P-384 JWK — claims verify (lines 2397-2399), key resolves (lines 2400-2402), hash equality (lines 2404-2416); as in P4, the hand-signed ES384 payload (lines 2377-2378) carries no embedded `at_hash` claim, so `from_token` is asserted against the spec-defined expected value — the base64url left half of SHA-384 over the access token (lines 2417-2424). |
| P6 | Observed — battery: `test_id_token_verification_key_at_hash_rsa_pss_eddsa`, `RsaSsaPkcs1V15Sha256` iteration of the RS/PSS loop (lines 2663-2683) through helper `assert_jwks_resolved_at_hash` — signs (lines 2550-2557), verifies claims (lines 2566-2569), resolves the matching JWK (lines 2572-2574), hashes identically to the fixture key (lines 2578-2584), reproduces the `at_hash` (lines 2586-2592). |
| P7 | Observed — battery: same helper, `RsaSsaPkcs1V15Sha384` iteration (lines 2663-2683, helper 2550-2592). |
| P8 | Observed — battery: same helper, `RsaSsaPkcs1V15Sha512` iteration (lines 2663-2683, helper 2550-2592). |
| P9 | Observed — battery: same helper, `RsaSsaPssSha256` iteration (lines 2663-2683, helper 2550-2592). |
| P10 | Observed — battery: same helper, `RsaSsaPssSha384` iteration (lines 2663-2683, helper 2550-2592). |
| P11 | Observed — battery: same helper, `RsaSsaPssSha512` iteration (lines 2663-2683, helper 2550-2592). |
| P12 | Observed — battery: EdDSA leg — Ed25519 fixture key (lines 2686-2690), `EdDsa` iteration through the same helper (lines 2698-2709). |
| P13 | Observed — battery: the HS256 missing-secret negative — `verification_key` against a public verifier holding no client secret returns `Err(SignatureVerificationError::DisallowedAlg(_))` rather than panicking (lines 1863-1872). Manual: `src/verification/mod.rs::JwtClaimsVerifier::verification_key` lines 502-511 is the single shared-secret branch, selected by `uses_shared_secret()` (= `kty` Symmetric per `src/core/mod.rs` lines 689-692, true for HS256, HS384, and HS512 alike); with `client_secret` `None` it returns the same `DisallowedAlg` error (lines 508-510), so HS384/HS512 inherit the decided negative. |
| P14 | Observed — battery: a substituted access token produces an `assert_ne!`-checked different hash in every named family — HS256 (lines 1852-1859), ES256 (lines 1941-1948), HS384 and HS512 (lines 2325-2332, both loop iterations), ES384 (lines 2426-2433), and all six RSA plus the EdDSA iterations (lines 2594-2601, helper exercised at lines 2663-2683 and 2698-2709). The comparison baseline is the embedded `at_hash` claim for the HS/RSA/EdDSA families and the spec-defined expected hash described in P4/P5 for ES256/ES384. |
| P15 | Observed — manual: `src/id_token/mod.rs::IdToken::signing_key` (lines 169-179) returns a borrowed `&'s K` and resolves exclusively through `JwtClaimsVerifier::signing_key` (`src/verification/mod.rs` lines 459-491, provider key-set filter only); its documentation (lines 159-165) states the JWKS-only scope and redirects shared-secret algorithms to `verification_key`. |
| P16 | Observed — doc-test battery: both crate-root tutorials compile against `verification_key` followed by `AccessTokenHash::from_token` with `reqwest-blocking` enabled — the synchronous tutorial doctest `src/lib.rs - (line 134)` (call chain at `src/lib.rs` lines 230-233) and the asynchronous tutorial doctest `src/lib.rs - (line 510)` (call chain at lines 608-611); `test result: ok. 7 passed; 0 failed; 2 ignored`. |
| F1 | Observed — battery plus manual: the shared-secret branch never falls back to a provider JWKS key — `src/verification/mod.rs` lines 502-511 returns the `DisallowedAlg` error without consulting the key set (the JWKS resolution at line 512 is reachable only for non-shared-secret algorithms) — and the missing-secret case errors rather than being accepted, exercised at `src/verification/tests.rs` lines 1863-1872 and decided for HS384/HS512 by the same shared branch as P13. |
| F2 | Observed — battery: nonmatching-JWK negatives for every named asymmetric family — ES256: a JWKS holding only a nonmatching P-256 key fails claims verification and key resolution with `SignatureVerificationError::NoMatchingKey` (lines 1950-1984); ES384: same negative (lines 2435-2469); RS256/384/512, PS256/384/512, and EdDSA: the helper's nonmatching-JWK branch asserts `NoMatchingKey` for claims and key resolution (lines 2603-2622, exercised by every iteration at lines 2663-2683 and 2698-2709). Hash reproduction is asserted only against the key resolved from the matching JWKS (lines 2578-2592), so a resolved nonmatching key cannot satisfy the positive assertions. |
| F3 | Observed — battery: the P14 `assert_ne!` assertions fail if a substituted access token compared equal to the family's at_hash baseline in any named family (same assertions as P14, including the ES256/ES384 spec-defined baselines). |
| F4 | Observed — doc-test battery: the tutorials still compile against the owned-key `verification_key` call; both doctests report `compile ... ok` (log lines `test src/lib.rs - (line 134) - compile ... ok` and `test src/lib.rs - (line 510) - compile ... ok`). |
| F5 | Observed — manual: the same inspection as P15 — `signing_key` keeps the documented borrowed (`&'s K`) JWKS-only scope (`src/id_token/mod.rs` lines 159-179); no shared-secret derivation exists on its path (`src/verification/mod.rs` lines 459-491). |
