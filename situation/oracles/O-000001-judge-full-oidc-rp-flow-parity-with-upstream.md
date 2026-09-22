# O-000001 — Judge full OIDC RP flow parity with upstream

## State

designed

## Judges

P-000001

## Inputs

An integration test matrix exercising every flow against a real OIDC
provider, the same inputs run against the upstream openidconnect 4.0.1
crate, and the upstream test suite.

## Pass

- P1: Discovery document fetched and parsed (issuer, endpoints, JWKS URI,
  supported algorithms)
- P2: Authorization-code flow with PKCE S256 completes (auth URL, callback,
  token exchange)
- P3: ID token verified for RS256, RS384, RS512, PS256, PS384, PS512, EdDSA
- P4: UserInfo endpoint fetched and parsed
- P5: Refresh token exchanged for new access token
- P6: All upstream test suite tests pass unchanged (excluding renamed
  imports)

## Fail

- F1: Any upstream-supported flow does not work
- F2: Any upstream test that passed before now fails

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item O-000001
(project Status: Todo).
