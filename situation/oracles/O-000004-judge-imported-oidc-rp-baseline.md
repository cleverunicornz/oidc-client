# O-000004 — Judge imported OIDC RP baseline parity

## State

designed

## Judges

situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md

## Inputs

A real-OIDC-provider integration matrix covering the in-scope flows, run with
the same inputs against this crate and openidconnect 4.0.1, plus this crate's
imported offline test suite at the tested head.

## Pass

- P1: Both crates fetch and parse equivalent discovery metadata, including
  issuer, endpoints, JWKS URI, and the listed signing algorithms.
- P2: Both crates complete authorization-code flow with PKCE S256 through
  authorization request, callback, and token exchange.
- P3: Both crates validate ID tokens using RS256, RS384, RS512, PS256, PS384,
  PS512, and EdDSA.
- P4: Both crates fetch and parse a UserInfo response.
- P5: Both crates exchange a refresh token for a new access token.
- P6: The imported offline test suite passes at the tested head, including the
  dynamic-registration behavior covered by that suite. This is execution
  evidence only; it does not claim that the test bytes are unchanged from the
  upstream pin.

## Fail

- F1: Any P1–P5 in-scope flow differs between this crate and openidconnect
  4.0.1 for the same provider input.
- F2: The imported offline test suite fails at the tested head, including a
  dynamic-registration test.

## References

- Supersedes the pre-correction rule in
  `situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md`;
  its existing W-000001 observation remains attached to that historical rule.
- `situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`
  records the classified non-rename test deltas that preclude an unchanged-suite
  claim.
- `situation/gaps/G-000005-no-crate-provided-discovery-cache.md`
