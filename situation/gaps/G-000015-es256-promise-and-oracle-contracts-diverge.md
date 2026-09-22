# G-000015 — ES256 Promise and Oracle contracts aligned

## State

closed

## Gap

At the reviewed head, P-000002 explicitly promised ES256 signature
verification, EC P-256 key acceptance, and discovery-metadata parsing, while
O-000002 additionally judged issuer, audience, nonce, expiry, and
mismatched-curve behavior. The Promise's Residual called claim checks
unassured without making them an explicit clause, and the Promise stated the
internal `p256` implementation choice as behavior.

## Relevance

O-000002 must collectively decide every explicit in-scope clause of P-000002
without broadening it, while a Promise must state observable behavior rather
than implementation detail. These records defined the ES256 assurance boundary
and required a coherent correction.

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

At the reviewed head, a complete future Witness could not be judged coherently:
it would either have to satisfy claim-validation and wrong-curve obligations
that the Promise did not state, or ignore Oracle legs that currently gated
assurance. The internal crate choice could also be mistaken for promised
consumer behavior.

## Resolution

closed — `9dd7999` states public verifier claim validation in P-000002 and
removes the internal implementation choice; `4c32b62` aligns O-000002 to that
scope and excludes mismatched-curve behavior; `7881bd7` makes the retained
W-000002 observation match the corrected rule.

## References

- `situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`
- `situation/oracles/O-000002-judge-es256-verification.md`
- `situation/gaps/G-000009-es256-oracle-overstates-wrong-curve-test-coverage.md`
- `situation/promises/AGENTS.md`
- `situation/oracles/AGENTS.md`
