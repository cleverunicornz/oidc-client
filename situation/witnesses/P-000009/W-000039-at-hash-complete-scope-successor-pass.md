# W-000039 — at_hash complete-scope successor PASS

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md

## Result

PASS — every Pass and Fail leg of O-000025 is decided at this head: the
executable legs by the retained battery runs below, the manual legs by the
inspections cited per row. This is the complete-scope successor's own
observation at the repaired head, and it completes the observation its
predecessor could not make: commit `c650c9d` repairs the ES256/ES384 fixtures
so their hand-signed payloads embed an `at_hash` (byte-identical to what
`CoreIdToken::new(..., Some(&access_token), ...)` embeds; no ECDSA
`PrivateSigningKey` impl exists to sign through it directly), assert the
documented flow reproduces the embedded `claims.access_token_hash()`, and
assert a substituted access token differs from the embedded claim — closing
the defect W-000037 recorded. W-000037 remains its own immutable INVALID
observation at `c5260aeb4a426f7881c8261ef2101d5a30710c29` and is not composed
into this one. P-000009's canonical State and the related open Gaps are frozen
by the fourth closure; Promise-state transitions and Gap resolutions belong to
the next closure's corrector and are not performed here.

## Head

c650c9dc1dfbe156523230dc86e52c72cf1a952d

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000039/cargo-test-id-token-verification-key-at-hash.log`
  retains `cargo test --offline --lib --quiet --
  id_token_verification_key_at_hash` (`exit=0`): `test result: ok. 4 passed;
  0 failed; 0 ignored; 0 measured; 108 filtered out; finished in 0.14s`. It
  executes all four named fixtures: `test_id_token_verification_key_at_hash`,
  `test_id_token_verification_key_at_hash_hs384_hs512`,
  `test_id_token_verification_key_at_hash_es384`, and
  `test_id_token_verification_key_at_hash_rsa_pss_eddsa`. SHA-256
  `f7621df30b08f91e012dd6f6531b9a38270ec38b2da8d23fb288106348f79f9b`.
- `situation/witnesses/evidence/W-000039/cargo-test-doc-reqwest-blocking.log`
  retains `cargo test --offline --doc --features reqwest-blocking` (`exit=0`):
  `test result: ok. 7 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out;
  finished in 0.18s`. Both crate-root tutorials compile in this run as
  `test src/lib.rs - (line 134) - compile ... ok` and
  `test src/lib.rs - (line 510) - compile ... ok`. SHA-256
  `3489066f59441ed17ab7f52be16248e2046800095128e67ab9f3cb700f4a66bb`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `c650c9dc1dfbe156523230dc86e52c72cf1a952d`.
- The ES256/ES384 fixtures hand-sign because no ECDSA `PrivateSigningKey`
  impl exists. The embedded `at_hash` is derived from the provider JWK with
  `hash_bytes` and truncated exactly as `CoreIdToken::new` embeds it
  (`src/verification/tests.rs` lines 1891-1899 and 2386-2394), so the
  fixtures' embedded claim and the crate's own embedding agree byte-for-byte.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness decides the successor oracle's legs at this head
  only; it performs no Promise-state transition and resolves no Gap.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | Observed — battery: the HS256 ID token is signed by `CoreIdToken::new` with `Some(&access_token)`, so the crate embeds the `at_hash` (`src/verification/tests.rs` lines 1817-1824); it verifies through the confidential verifier with an empty JWKS (lines 1829-1839), `verification_key` resolves from the client secret (lines 1841-1843), and `AccessTokenHash::from_token` reproduces the embedded hash (`assert_eq!` at line 1850, claims-derived expected value at line 1840). |
| P2 | Observed — battery: `test_id_token_verification_key_at_hash_hs384_hs512`, `HmacSha384` iteration of the algorithm loop (lines 2296-2343): token signed with an `at_hash` (lines 2301-2308), confidential verifier with empty JWKS (lines 2312-2319), claims verify (lines 2320-2323), key resolves from the client secret (lines 2324-2326), `from_token` reproduces the embedded hash (lines 2327-2333). |
| P3 | Observed — battery: same fixture, `HmacSha512` iteration (lines 2296-2343). |
| P4 | Observed — battery: the repaired ES256 leg embeds an `at_hash` in the signed payload — fixture hash computed from the provider JWK with `hash_bytes` (lines 1893-1898), half-truncated `AccessTokenHash` (line 1899), payload embedding `"at_hash"` (lines 1901-1906), hand-signed (lines 1907-1916); claims verify against the matching P-256 JWK and the embedded claim is read as `es256_claims.access_token_hash()` (lines 1918-1929); `verification_key` resolves the provider JWK from the JWKS (lines 1930-1932); the documented call reproduces the token's EMBEDDED claim — `assert_eq!(es256_at_hash, embedded_at_hash)` (lines 1941-1947) — with the cross-checks that the resolved key hashes exactly like the fixture key (lines 1934-1940) and that the embedded claim equals the independently computed `es256_expected` (lines 1948-1949). |
| P5 | Observed — battery: the repaired ES384 leg, same structure in `test_id_token_verification_key_at_hash_es384`: P-384 fixture key (lines 2369-2384), payload embedding `at_hash` (lines 2386-2411), claims verify and embedded claim read (lines 2413-2424), key resolution from the JWKS (lines 2425-2427), resolved-key hash cross-check (lines 2429-2435), `assert_eq!(es384_at_hash, embedded_at_hash)` (lines 2436-2442), embedded-vs-independent cross-check (lines 2443-2444). |
| P6 | Observed — battery: `test_id_token_verification_key_at_hash_rsa_pss_eddsa`, `RsaSsaPkcs1V15Sha256` iteration of the RS/PSS loop (lines 2683-2703) through helper `assert_jwks_resolved_at_hash` (lines 2547-2643) — `CoreIdToken::new` signs with the access token so the crate embeds the `at_hash` (lines 2570-2577), claims verify against the matching JWK (lines 2586-2589), `verification_key` resolves it (lines 2592-2594), the resolved key hashes exactly like the fixture key (lines 2598-2604), and `from_token` reproduces the embedded hash (lines 2606-2612). |
| P7 | Observed — battery: same helper, `RsaSsaPkcs1V15Sha384` iteration (lines 2683-2703, helper 2547-2643). |
| P8 | Observed — battery: same helper, `RsaSsaPkcs1V15Sha512` iteration (lines 2683-2703, helper 2547-2643). |
| P9 | Observed — battery: same helper, `RsaSsaPssSha256` iteration (lines 2683-2703, helper 2547-2643). |
| P10 | Observed — battery: same helper, `RsaSsaPssSha384` iteration (lines 2683-2703, helper 2547-2643). |
| P11 | Observed — battery: same helper, `RsaSsaPssSha512` iteration (lines 2683-2703, helper 2547-2643). |
| P12 | Observed — battery: EdDSA leg — Ed25519 fixture key (lines 2706-2710), `EdDsa` call through the same helper (lines 2718-2729). |
| P13 | Observed — battery plus manual: the HS256 missing-secret negative — `verification_key` against a public verifier holding no client secret returns `Err(SignatureVerificationError::DisallowedAlg(_))` rather than panicking (lines 1863-1872). Manual: `src/verification/mod.rs::JwtClaimsVerifier::verification_key` lines 497-513 is the single shared-secret branch, selected by `uses_shared_secret()` (= `kty` Symmetric per `src/core/mod.rs` lines 689-692, true for HS256, HS384, and HS512 alike); with `client_secret` `None` it returns the same `DisallowedAlg` error (lines 508-510), so HS384/HS512 inherit the decided negative. |
| P14 | Observed — battery: substituted-token inequality against embedded `at_hash` claims for every named family — HS256 (lines 1852-1859), HS384/HS512 (lines 2335-2342), ES256 against the embedded claim (lines 1951-1958), ES384 against the embedded claim (lines 2446-2453), and the RSA/PSS/EdDSA helper iterations (lines 2614-2621, exercised at lines 2683-2703 and 2718-2729). |
| P15 | Observed — manual: `src/id_token/mod.rs::IdToken::signing_key` (lines 169-179) returns a borrowed `&'s K` and resolves exclusively through `JwtClaimsVerifier::signing_key` (`src/verification/mod.rs` lines 459-491, provider key-set filter at line 465); its documentation (lines 159-165) states the JWKS-only scope and redirects shared-secret algorithms to `verification_key`. |
| P16 | Observed — doc-test battery: both crate-root tutorials compile against `verification_key` followed by `AccessTokenHash::from_token` with `reqwest-blocking` enabled — the synchronous tutorial doctest `src/lib.rs - (line 134)` (call chain at `src/lib.rs` lines 229-238) and the asynchronous tutorial doctest `src/lib.rs - (line 510)` (call chain at lines 607-616); `test result: ok. 7 passed; 0 failed; 2 ignored`. |
| F1 | Observed — battery plus manual: the shared-secret branch never falls back to a provider JWKS key — `src/verification/mod.rs` lines 502-511 returns the client-secret-derived key or the `DisallowedAlg` error without consulting the key set (the JWKS resolution at line 512 is reachable only for non-shared-secret algorithms) — and the missing-secret case errors rather than being accepted, exercised at `src/verification/tests.rs` lines 1863-1872 and decided for HS384/HS512 by the same shared branch as P13. |
| F2 | Observed — battery: nonmatching-JWK negatives for every named asymmetric family — ES256: a JWKS holding only a nonmatching P-256 key fails claims verification and key resolution with `SignatureVerificationError::NoMatchingKey` (lines 1960-1994: claims 1985-1990, key resolution 1991-1994); ES384: same negative (lines 2455-2489: claims 2480-2485, key resolution 2486-2489); RS256/384/512, PS256/384/512, and EdDSA: the helper's nonmatching-JWK branch asserts `NoMatchingKey` for claims (lines 2633-2638) and key resolution (lines 2639-2642), exercised by every iteration at lines 2683-2703 and 2718-2729. Hash reproduction is asserted only against the key resolved from the matching JWKS (cross-checks at lines 1940, 2435, and 2604), so a resolved nonmatching key cannot satisfy the positive assertions. |
| F3 | Observed — battery: the substituted-token assertions now compare against embedded `at_hash` claims in every named family (same coverage as P14, ES256 at lines 1951-1958 and ES384 at lines 2446-2453), so the any-family failure condition is decided. |
| F4 | Observed — doc-test battery: the tutorials still compile against the owned-key `verification_key` call; both doctests report `compile ... ok` (log lines `test src/lib.rs - (line 134) - compile ... ok` and `test src/lib.rs - (line 510) - compile ... ok`). |
| F5 | Observed — manual: the same inspection as P15 — `signing_key` keeps the documented borrowed (`&'s K`) JWKS-only scope (`src/id_token/mod.rs` lines 159-179); no shared-secret derivation exists on its path (`src/verification/mod.rs` lines 459-491). |
