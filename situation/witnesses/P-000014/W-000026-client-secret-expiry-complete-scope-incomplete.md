# W-000026 — Registration client-secret expiry complete-scope observation is incomplete

## Promise

situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md

## Oracle

situation/oracles/O-000024-judge-client-secret-expiration-complete-scope.md

## Result

INVALID — the fresh feature-gated bounded command passed the retained concrete
three-state and collision fixtures at the current head, but it did not
independently observe the generic non-colliding numeric domain, feature-gated
non-colliding RFC3339 acceptance, or shared-`Timestamp`-consumer isolation.
This is an incomplete observation of the complete-scope rule, not a partial
PASS and not assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000026/cargo-test-client-secret-expiration-complete-scope.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --features
  accept-rfc3339-timestamps client_secret_expiration_` at this head (`exit=0`).
  Its SHA-256 is
  `ead237f9e8b2f537fd004116cbadb993fffc900e16574e6b1b72776681043bf3`
  in the adjacent `SHA256SUMS` file.
- The command selected the concrete core and epoch-collision fixtures. It did
  not include a non-colliding RFC3339 acceptance fixture or a direct observation
  of the entire non-colliding numeric domain and unchanged adapter consumers.
- W-000017 remains the PASS observation of superseded O-000015, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the numeric-`0` `NeverExpires` fixture passed in the retained command. |
| P2 | PASS — the absent-field fixture passed in the retained command. |
| P3 | INVALID — the one concrete numeric fixture does not decide every non-colliding numeric timestamp. |
| P4 | INVALID — no non-colliding RFC3339 acceptance fixture or manual observation was retained. |
| P5 | PASS — the three concrete state round-trip fixtures passed in the retained command. |
| P6 | PASS — the setter fixture passed in the retained command. |
| P7 | INVALID — no shared-adapter isolation observation was retained for this successor rule. |
