# W-000003 — Full ES256 ID-token fixture through the public verifier

## Promise

situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md

## Oracle

situation/oracles/O-000002-judge-es256-verification.md

## Result

PASS — a real pinned-container `cargo test --all-features` run at the head
below executes every O-000002 Pass leg, including the previously manual P4
leg: the new fixture `verification::tests::test_es256_id_token_verified_claims`
signs a compact ID token with a self-generated deterministic P-256 key under
an `ES256` JOSE header and verifies it through the public
`CoreIdTokenVerifier::new_public_client` path (with ES256 selected via the
public `set_allowed_algs` builder), accepting the valid token and rejecting
wrong issuer, wrong audience, wrong nonce, expired, tampered-signature, and
wrong-key variants (the F4 rejection decisions). This completes the evidence
W-000002 lacked; that witness remains the retained historical observation.

## Head

52e84d5dff27ac026ac7670370e3dda918cbeaf7

## Observed

2026-09-22

## Evidence

`situation/witnesses/evidence/W-000003/gates-es256-fixture.log` retains the
run; its digest and the container identity are recorded in the neighboring
`SHA256SUMS`. The run executes 72 unit tests (all passing), the 21
live-network certification tests remaining ignored, and 7 passing doc tests
(2 ignored). Besides the new fixture, the same run passes
`test_ecdsa_verification`, `test_core_jwk_deserialization_ec`, and
`test_discovery_deserialization`, which decide P1–P3 exactly as recorded in
W-000002.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_ecdsa_verification` passed in `gates-es256-fixture.log`; it accepts the valid P-256 fixture under direct ES256 enum dispatch. |
| P2 | PASS — `test_core_jwk_deserialization_ec` and `test_ecdsa_verification` passed in `gates-es256-fixture.log`; the new fixture additionally parses a self-generated EC P-256 JWK (`kty`/`crv`/`x`/`y`) that the verifier accepts for key selection. |
| P3 | PASS — `test_discovery_deserialization` passed in `gates-es256-fixture.log` with a fixture that contains ES256. |
| P4 | PASS — `test_es256_id_token_verified_claims` passed in `gates-es256-fixture.log`: valid ES256 ID token verifies signature, issuer, audience, nonce, and expiry through the public verifier; the same test rejects wrong issuer, wrong audience, wrong nonce, expired, tampered signature, and wrong-key tokens (F1–F4 rejection behavior decided for this fixture). |
