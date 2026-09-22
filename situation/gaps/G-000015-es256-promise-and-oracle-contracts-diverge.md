# G-000015 — ES256 Promise and Oracle contracts diverge

## State

open

## Gap

P-000002 explicitly promises ES256 signature verification, EC P-256 key
acceptance, and discovery-metadata parsing, but O-000002 additionally judges
issuer, audience, nonce, and expiry behavior and mismatched-curve rejection.
The Promise's Residual calls the claim checks unassured without first making
them an explicit Promise clause, and the Promise states the internal `p256`
implementation choice as behavior.

## Relevance

O-000002 must collectively decide every explicit in-Scope clause of P-000002
without broadening it, while a Promise must state observable behavior rather
than implementation detail. These records were substantively rewritten in the
reviewed closure and define the ES256 assurance boundary.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- P-000002's Promise section says the crate verifies the ES256 JWS signature,
  accepts the matching JWK during key selection, uses `p256`, and parses ES256
  discovery metadata. Its Scope says it covers signature verification and
  metadata parsing.
- O-000002 P4/F4 additionally require acceptance and rejection decisions for
  issuer, audience, nonce, and expiry through the public ID-token verifier.
- O-000002 F3 also requires mismatched-curve rejection, although P-000002's
  positive Scope is limited to matching EC P-256 public keys; G-000009
  separately records that the named test does not executablely decide that
  portion.
- `situation/promises/AGENTS.md` requires Promise behavior rather than
  implementation detail, and `situation/oracles/AGENTS.md` prohibits judging
  behavior outside the Promise's explicit Scope.

## Impact

A complete future Witness cannot be judged coherently: it would either have to
satisfy claim-validation and wrong-curve obligations that the Promise does not
state, or ignore Oracle legs that currently gate assurance. Conversely, the
internal crate choice could be mistaken for consumer-observable promised
behavior.

## Resolution

none

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/oracles/O-000002-judge-es256-verification.md`
- `situation/gaps/G-000009-es256-oracle-overstates-wrong-curve-test-coverage.md`
- `situation/promises/AGENTS.md`
- `situation/oracles/AGENTS.md`
