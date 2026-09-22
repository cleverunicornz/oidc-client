# P-000001 — Imported OIDC RP surface retains the openidconnect 4.0.1 baseline

## State

implemented

## Promise

This crate carries the openidconnect 4.0.1 OIDC relying-party baseline:
provider discovery and metadata parsing, authorization-code flow construction
with PKCE S256, ID-token verification for RS256/384/512, PS256/384/512, and
EdDSA, UserInfo requests, refresh-token requests, and dynamic client
registration. The carried behavior is intended to remain identical to the
upstream baseline within this promise's scope.

## Scope

The imported 4.0.1 OIDC RP surface and the listed non-ES256 algorithms.
ES256 is separately described by P-000002. This scope does not include a
crate-provided discovery cache, provider deployment behavior, publication, or
any algorithm not listed above. It does not claim that the imported test bytes
are unchanged from the pin.

## Oracle

situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md

## State evidence

State `implemented` is supported by the import commit
`eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a`. The retained gate observation at
`24835e4b44caa8a0baae2ac5b865bbd6bdf355ba` records
`cargo test --all-features` with 70 passed and 0 failed tests, while the 21
live-network certification tests compiled and remained ignored
(`situation/witnesses/evidence/W-000001/gates-final.log`). Its witness is
`INVALID`, remains attached to the historical O-000001 rule, and establishes
only that the suite passed at that head; it does not establish unchanged test
bytes. D-000004 records classified non-rename test deltas. The corrected scope
is judged by O-000004 and has no PASS witness, so this promise is not
`assured`.

## Residual

Live-flow parity against a real OIDC provider remains unassured for discovery,
authorization-code exchange, ID-token validation, UserInfo, and refresh
exchange. Discovery caching is outside scope and retained as
G-000005; ES256 is outside this promise and retained by P-000002.

## References

- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
- situation/gaps/G-000005-no-crate-provided-discovery-cache.md
- situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md
