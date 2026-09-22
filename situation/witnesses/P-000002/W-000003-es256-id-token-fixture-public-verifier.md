# W-000003 — Full ES256 ID-token fixture through the public verifier

## Promise

situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Oracle

situation/oracles/O-000002-judge-es256-verification.md

## Result

PASS — a real pinned-container `cargo test --all-features` run at the head
below executes every O-000002 Pass leg, including the previously manual P4
leg: the fixture `verification::tests::test_es256_id_token_verified_claims`
signs a compact ID token with a self-generated deterministic P-256 key under
an `ES256` JOSE header, parses it through the public consumer entry
(`IdToken: FromStr` into `CoreIdToken`), and verifies it with the public
`CoreIdToken::claims` against a `CoreIdTokenVerifier::new_public_client`
verifier (ES256 selected via the public `set_allowed_algs` builder),
accepting the valid token and rejecting wrong issuer, wrong audience, wrong
nonce, expired, tampered-signature, and wrong-key variants (the F4 rejection
decisions). This completes the evidence W-000002 lacked; that witness
remains the retained historical observation.

Correction (open PR #2, before any closing checkpoint): the first version of
this witness (head `52e84d5dff27ac026ac7670370e3dda918cbeaf7`) recorded a
fixture that entered through the crate-internal verifier
`verified_claims` method. The independent validator flagged the entry point;
commit `00d45a7` rerouted the fixture through the public `CoreIdToken`
entry and this witness was corrected in place against the retained rerun.

## Head

00d45a7c06f3704915550ad96610a9f9d5e68279

## Observed

2026-09-22

## Evidence

`situation/witnesses/evidence/W-000003/gates-es256-fixture.log` retains the
rerun (sha256 `2d95bede4bdd2e0402540be283204ef717b9d90de85efe2b01094aadf0cf0a89`,
recorded with the container identity in the neighboring `SHA256SUMS`; the
log header names the same head). The run executes 72 unit tests (all
passing), the 21 live-network certification tests remaining ignored, and 7
passing doc tests (2 ignored). Besides the fixture, the same run passes
`test_ecdsa_verification`, `test_core_jwk_deserialization_ec`, and
`test_discovery_deserialization`, which decide P1–P3 exactly as recorded in
W-000002.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_ecdsa_verification` passed in `gates-es256-fixture.log`; it accepts the valid P-256 fixture under direct ES256 enum dispatch. |
| P2 | PASS — `test_core_jwk_deserialization_ec` and `test_ecdsa_verification` passed in `gates-es256-fixture.log`; the new fixture additionally parses a self-generated EC P-256 JWK (`kty`/`crv`/`x`/`y`) that the verifier accepts for key selection. |
| P3 | PASS — `test_discovery_deserialization` passed in `gates-es256-fixture.log` with a fixture that contains ES256. |
| P4 | PASS — `test_es256_id_token_verified_claims` passed in `gates-es256-fixture.log`: valid ES256 ID token, parsed via the public `IdToken: FromStr`, verifies signature, issuer, audience, nonce, and expiry through the public `CoreIdToken::claims` entry; the same test rejects wrong issuer, wrong audience, wrong nonce, expired, tampered signature, and wrong-key tokens (F1–F4 rejection behavior decided for this fixture). |
