# G-000014 — README logout claim removed for lack of local lineage

## State

closed

## Gap

At the reviewed head, the rewritten human README presented logout as a current
crate capability, but no local Promise explicitly claimed logout and no Oracle
judged it. P-000001's enumerated baseline behavior and O-000004's flow matrix
omitted logout.

## Relevance

README.md changed in this BACKPORT and pointed readers to P-000001 as the
canonical imported-baseline record. The unsupported current capability claim
needed traversable local Promise/Oracle lineage or removal from orientation.

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

At the reviewed head, the human orientation made a consumer-facing capability
claim whose current contract and assurance boundary could not be found through
the record system. Keeping it would either overstate the enumerated baseline
or require an unassigned broadening of P-000001/O-000004.

## Resolution

closed — `7901344` removes logout from the current README capability list
rather than inventing Promise/Oracle lineage for it.

## References

- `README.md`
- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
- `situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md`
- `src/logout.rs`
