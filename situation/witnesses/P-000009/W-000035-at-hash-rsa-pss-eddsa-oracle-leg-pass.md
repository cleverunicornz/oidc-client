# W-000035 — at_hash RSA/RSA-PSS/EdDSA Oracle-leg observation passes

## Promise

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Oracle

situation/oracles/O-000025-judge-documented-at-hash-rsa-pss-eddsa-families.md

## Result

PASS — every O-000025 leg is decided at this head. The fixture executes all
seven judged families (RS256/384/512, PS256/384/512, EdDSA) through the same
documented helper, and every iteration asserts signature verification, JWKS
resolution through `IdToken::verification_key`, resolved-key/fixture-key hash
equivalence, `AccessTokenHash::from_token` reproduction of the embedded
`at_hash`, and the substituted-token negative. Together with W-000028 (the
observation of O-000019's listed legs), this supplies recorded PASS coverage
for P-000009's named clauses. P-000009's frozen canonical State remains
`implemented`, as retained by G-000032; this Witness does not change it.

## Head

48761b267c479b38918e4a139ed5c8fd530f2236

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000035/cargo-test-at-hash-rsa-pss-eddsa-fixture.log`
  retains `cargo test --offline --lib --quiet -- test_id_token_verification_key_at_hash_rsa_pss_eddsa`
  (`exit=0`): `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured;
  111 filtered out; finished in 0.14s`. SHA-256
  `ddbfe0051704ee4deb9a266af9a85552522e72e2248af504e51eb5711589c65e`.
- `situation/witnesses/evidence/W-000035/cargo-test-at-hash-filter-battery.log`
  retains the shared battery `cargo test --offline --lib --quiet --
  id_token_verification_key_at_hash` (`exit=0`): `test result: ok. 4 passed;
  0 failed; 0 ignored; 0 measured; 108 filtered out; finished in 0.14s`,
  corroborating that the battery now selects four fixtures — the three
  O-000019 fixtures plus this oracle's. SHA-256
  `f7621df30b08f91e012dd6f6531b9a38270ec38b2da8d23fb288106348f79f9b`.
- Digests are recorded in the adjacent `SHA256SUMS` and were re-verified with
  `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `48761b267c479b38918e4a139ed5c8fd530f2236`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This Witness observes one Promise under one Oracle at one head; it
  does not change P-000009's frozen canonical `implemented` State, retained by
  G-000032.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the fixture's `RsaSsaPkcs1V15Sha256` iteration: `CoreIdToken::new(..., Some(access_token), None)` embeds the `at_hash` (`src/verification/tests.rs` lines 2421-2428); `id_token.claims(&verifier, nonce)` succeeds against the public client verifier whose JWKS publishes the RSA JWK with the algorithm allowed (lines 2430-2439); `IdToken::verification_key` resolves the JWK from the JWKS (lines 2443-2445); resolved-key and fixture-key `hash_bytes` agree (lines 2449-2455); `AccessTokenHash::from_token` over the resolved key equals the embedded hash (lines 2457-2463). |
| P2 | PASS — same fixture and helper, `RsaSsaPkcs1V15Sha384` iteration of the algorithm loop (lines 2504-2523). |
| P3 | PASS — same fixture, `RsaSsaPkcs1V15Sha512` iteration (lines 2504-2523). |
| P4 | PASS — same fixture, `RsaSsaPssSha256` iteration (lines 2504-2523); the PSS variants sign through the same `CoreRsaPrivateSigningKey` fixture (lines 2496-2503). |
| P5 | PASS — same fixture, `RsaSsaPssSha384` iteration (lines 2504-2523). |
| P6 | PASS — same fixture, `RsaSsaPssSha512` iteration (lines 2504-2523). |
| P7 | PASS — `EdDsa` iteration signs with `CoreEdDsaPrivateSigningKey::from_ed25519_pem` and publishes its OKP verification JWK (lines 2525-2541); the helper's verification, resolution, hash-equivalence, and reproduction assertions run unchanged. |
| P8 | PASS (negative executed) — every iteration computes `AccessTokenHash::from_token` over `AccessToken::new("substituted_access_token")` and `assert_ne!`s it against the embedded hash (lines 2465-2472), seven times across the seven iterations. |
| F1 | PASS (negative executed and structural) — executed: `IdToken::verification_key(&verifier).expect(...)` (`src/verification/tests.rs` lines 2443-2445) and `assert_eq!(from_resolved, from_fixture)` (lines 2449-2455) fail the fixture on any resolution failure or hash divergence; structural: `CoreJsonWebKey::hash_bytes` hashes only the supplied token bytes per family SHA (`src/core/jwk/mod.rs` lines 381-422) and ignores asymmetric key material, so hash reproduction alone does not distinguish keys — the nonmatching-JWK negative is carried by the same documented flow's signature verification, which every iteration runs first: `id_token.claims(&verifier, nonce)` succeeds only when the verifier's JWKS holds the matching key (lines 2437-2440). |
| F2 | PASS (negative executed) — `assert_eq!(actual_access_token_hash, expected_access_token_hash)` (lines 2457-2463) fails any family whose documented call fails to reproduce the embedded `at_hash`; all seven pass. |
| F3 | PASS (negative executed) — P8's `assert_ne!` (lines 2465-2472) fails any family in which the substituted token compares equal; all seven pass. |
