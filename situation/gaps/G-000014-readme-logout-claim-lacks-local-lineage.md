# G-000014 — README logout claim lacks local lineage

## State

open

## Gap

The rewritten human README presents logout as a current crate capability, but no
local Promise explicitly claims logout and no Oracle judges it. P-000001's
enumerated baseline behavior and O-000004's flow matrix omit logout.

## Relevance

README.md changed in this BACKPORT and points readers to P-000001 as the
canonical imported-baseline record. A current behavior introduced into the
human orientation by the reviewed interval needs traversable local
Promise/Oracle lineage or must be removed from the current capability list.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `README.md` lines 3–7 say the crate provides logout.
- P-000001 enumerates discovery, authorization-code construction, listed
  ID-token algorithms, UserInfo, refresh-token requests, and dynamic client
  registration; it does not name logout.
- O-000004 Pass legs cover discovery, authorization code, ID tokens, UserInfo,
  refresh tokens, and the imported offline suite's dynamic-registration
  behavior; no leg decides logout behavior.
- `src/logout.rs` establishes that implementation exists, but implementation is
  evidence rather than the missing local behavioral commitment.

## Impact

The changed README makes a consumer-facing capability claim whose current
contract and assurance boundary cannot be found through the record system. If
"the imported 4.0.1 OIDC RP surface" is intended to include logout implicitly,
then O-000004 is incomplete for that broad Scope; if the enumeration is the
actual boundary, the README overstates it.

## Resolution

none

## References

- `README.md`
- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
- `situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md`
- `src/logout.rs`
