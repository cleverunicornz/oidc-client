# G-000034 — JWK redaction Oracle omits the Clone contract

## State

closed

## Gap

P-000013 explicitly promises that `CoreJsonWebKey`'s `Clone` behavior is
unchanged alongside equality and serde behavior, but O-000023 has no Pass or
Fail leg for `Clone`. W-000032 therefore supplies listed-leg PASS evidence,
rather than complete-Promise evidence, because it has no Clone observation.

## Relevance

The reviewed DELTA adds W-000032 as a current listed-leg PASS observation. The
correction removes its complete-scope representation, but neither a successor
Oracle nor a valid Witness decides P-000013's Clone clause. The Oracle contract
requires every explicit in-Scope Promise clause to be decided; a visible
`#[derive(Clone)]` on the implementation is relevant structural evidence but
does not supply the missing predeclared Oracle leg or a corresponding Witness row.

## Evidence

- `situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md` states
  in its Promise section that "Clone, `PartialEq`/`Eq`, and serde serialization
  and deserialization of those fields are unchanged."
- `situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md` P5/F3
  judge equality and serde behavior; no Pass or Fail leg names Clone.
- `situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md`
  has one row for every listed O-000023 leg but contains no Clone evidence.
- `src/core/jwk/mod.rs` still derives `Clone` for `CoreJsonWebKey`; that source
  fact shows a plausible manual decision is available, not that the current
  Oracle declared or W-000032 applied it.
- Validator observation for closure run
  `20260924T094258Z-50bf95f61913526682a3550d07fa6b1d96d78935` at fixed
  reviewed head `8e61dc268952aa7f434cd164f7d0569a31113530` (2026-09-24).

- Closer reconciliation for closure run
  `20260924T111724Z-295cb33dbeb3b837300c31b0b1a17517095f2393` reviewed the
  admitted DELTA
  `7f1025e1b91c0f367f05528e7efb93846d437a81..295cb33dbeb3b837300c31b0b1a17517095f2393`.
  O-000026 declares Pass and Fail legs for the unchanged `Clone` clause, and
  W-000036 records PASS for every one of those legs at
  `48761b267c479b38918e4a139ed5c8fd530f2236`, with retained,
  checksum-verified fixture evidence.

## Impact

O-000023 does not collectively decide every explicit clause of P-000013, so
W-000032 cannot truthfully serve as complete-Promise assurance evidence for
that Promise. P-000013 remains `implemented` and unassured until a successor
Oracle has a valid Witness; G-000032 now distinguishes this listed-leg PASS
from sufficient complete-Promise coverage.

- Closer reconciliation closes the omitted-Clone concern: O-000026 and
  W-000036 now decide the previously absent `Clone` clause. P-000013's frozen
  canonical `implemented` State is a distinct reconciliation concern retained
  by G-000032.

## Resolution

closed — O-000026 and W-000036 supply implemented-Oracle and PASS-Witness
evidence for P-000013's previously omitted `Clone` clause at
`48761b267c479b38918e4a139ed5c8fd530f2236`. P-000013's frozen canonical State
remains a separate concern in G-000032.

## References

- situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
- situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-oracle-leg-pass.md
- situation/gaps/G-000032-current-pass-evidence-outpaces-frozen-promise-states.md
- situation/oracles/AGENTS.md
- situation/oracles/O-000026-judge-jwk-clone-contract.md
- situation/witnesses/P-000013/W-000036-jwk-clone-contract-oracle-leg-pass.md
