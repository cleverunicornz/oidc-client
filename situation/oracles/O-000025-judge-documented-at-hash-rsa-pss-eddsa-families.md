# O-000025 — Judge documented at_hash RSA, RSA-PSS, and EdDSA families

## State

implemented

## Judges

situation/promises/P-000009-documented-at-hash-flow-shared-secret-and-asymmetric.md

## Inputs

At the judged head, the fixture
`test_id_token_verification_key_at_hash_rsa_pss_eddsa`
(`src/verification/tests.rs`): one RSA private signing key exercised through
all six PKCS#1 v1.5 and RSA-PSS SHA-2 variants and one Ed25519 private signing
key, each published as the verification JWK of a public client verifier's
JWKS; per family, an ID token signed by `CoreIdToken::new` embedding the
`at_hash` of the access token, the documented flow
(`IdToken::verification_key` followed by `AccessTokenHash::from_token`), and
a substituted-access-token negative. This successor judges only the supported
asymmetric families O-000019's legs omit (RS256/384/512, PS256/384/512, and
EdDSA); with O-000019 it collectively decides P-000009's complete declared
Scope. The shared `id_token_verification_key_at_hash` filter battery selects
four fixtures; the three pre-existing ones remain O-000019's legs.

## Pass

- P1 — An RS256 ID token signed by the fixture RSA key verifies against the
  public client verifier whose JWKS publishes the matching JWK;
  `IdToken::verification_key` resolves that JWK; the resolved key hashes the
  access token exactly like the fixture verification key; and
  `AccessTokenHash::from_token` over the resolved key reproduces the embedded
  `at_hash`.
- P2 — The corresponding documented flow works for RS384.
- P3 — The corresponding documented flow works for RS512.
- P4 — The corresponding documented flow works for PS256.
- P5 — The corresponding documented flow works for PS384.
- P6 — The corresponding documented flow works for PS512.
- P7 — The corresponding documented flow works for EdDSA (Ed25519) through
  the crate's `CoreEdDsaPrivateSigningKey`.
- P8 — A substituted access token produces a different access-token hash than
  the token's `at_hash` in every judged family.

## Fail

- F1 — A judged family's flow cannot resolve its matching provider JWK
  through `IdToken::verification_key`, or the resolved key hashes the access
  token differently from the fixture verification key.
- F2 — `AccessTokenHash::from_token` over the resolved key fails to reproduce
  the embedded `at_hash` in any judged family.
- F3 — A substituted access token compares equal to the token's `at_hash` in
  any judged family.

## Implementation

`cargo test --offline --lib --quiet -- test_id_token_verification_key_at_hash_rsa_pss_eddsa` executes the fixture; every judged family runs the full
leg set above inside one fixture iteration of the shared helper.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | RS256 verifies, resolves its matching JWK, and reproduces the `at_hash`. | `src/verification/tests.rs::test_id_token_verification_key_at_hash_rsa_pss_eddsa` (`RsaSsaPkcs1V15Sha256` iteration) |
| P2 | RS384 likewise. | same fixture (`RsaSsaPkcs1V15Sha384` iteration) |
| P3 | RS512 likewise. | same fixture (`RsaSsaPkcs1V15Sha512` iteration) |
| P4 | PS256 likewise. | same fixture (`RsaSsaPssSha256` iteration) |
| P5 | PS384 likewise. | same fixture (`RsaSsaPssSha384` iteration) |
| P6 | PS512 likewise. | same fixture (`RsaSsaPssSha512` iteration) |
| P7 | EdDSA (Ed25519) likewise. | same fixture (`EdDsa` iteration) |
| P8 | Substituted token hashes differently in all seven iterations. | same fixture, `assert_ne!` over `"substituted_access_token"` per iteration |
| F1 | Nonmatching JWK resolution or resolved-key hash divergence fails the leg. | same fixture (negation of the resolution and `hash_bytes`-equality assertions) for resolution failure and hash divergence; structural for matching-key identity — `hash_bytes` hashes only the supplied token bytes per family SHA (`src/core/jwk/mod.rs` lines 381-422) and ignores asymmetric key material, so the nonmatching-JWK negative is carried by the flow's signature verification (`id_token.claims(...)`, `src/verification/tests.rs` lines 2437-2440), which every iteration runs first |
| F2 | Failure to reproduce the embedded `at_hash` fails the leg. | same fixture (negation of the `assert_eq!` over `AccessTokenHash::from_token`) |
| F3 | Substituted-token hash equality fails the leg. | same fixture (negation of P8's `assert_ne!`) |

## References

- Completes O-000019 for P-000009's complete declared scope: O-000019 remains
  in force for the legs it lists, and O-000019 plus O-000025 collectively
  decide the Promise's Scope — the shared-secret, ES256/ES384, `signing_key`
  scope, and documentation legs by O-000019, the remaining supported
  asymmetric families by this oracle. Neither supersedes the other.
- situation/gaps/G-000033-at-hash-oracle-omits-supported-asymmetric-algorithms.md
  — closed by this Oracle and W-000035, which decide its omitted-family
  concern. P-000009's frozen canonical State remains an independent
  reconciliation concern in G-000032.
- situation/witnesses/P-000009/W-000028-documented-at-hash-oracle-leg-pass.md
  remains the observation of O-000019's listed legs.
