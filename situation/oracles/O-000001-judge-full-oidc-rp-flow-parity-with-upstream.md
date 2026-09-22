# O-000001 — Judge full OIDC RP flow parity with upstream

## State

designed

## Judges

situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md

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

## References

- Superseded for P-000001's corrected scope by
  `situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md`. W-000001
  remains an observation of this historical rule.
