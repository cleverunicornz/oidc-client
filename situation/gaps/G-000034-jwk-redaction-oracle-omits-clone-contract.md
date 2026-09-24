# G-000034 — JWK redaction Oracle omits the Clone contract

## State

open

## Gap

P-000013 explicitly promises that `CoreJsonWebKey`'s `Clone` behavior is
unchanged alongside equality and serde behavior, but O-000023 has no Pass or
Fail leg for `Clone`. W-000032 likewise supplies no Clone observation while
calling the result complete-scope PASS.

## Relevance

The reviewed DELTA adds W-000032 as current complete-scope evidence for
P-000013. The Oracle contract requires every explicit in-Scope Promise clause
to be decided; a visible `#[derive(Clone)]` on the implementation is relevant
structural evidence but does not supply the missing predeclared Oracle leg or a
corresponding Witness row.

## Evidence

- `situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md` states
  in its Promise section that "Clone, `PartialEq`/`Eq`, and serde serialization
  and deserialization of those fields are unchanged."
- `situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md` P5/F3
  judge equality and serde behavior; no Pass or Fail leg names Clone.
- `situation/witnesses/P-000013/W-000032-jwk-debug-redaction-complete-scope-pass.md`
  has one row for every listed O-000023 leg but contains no Clone evidence.
- `src/core/jwk/mod.rs` still derives `Clone` for `CoreJsonWebKey`; that source
  fact shows a plausible manual decision is available, not that the current
  Oracle declared or W-000032 applied it.
- Validator observation for closure run
  `20260924T094258Z-50bf95f61913526682a3550d07fa6b1d96d78935` at fixed
  reviewed head `8e61dc268952aa7f434cd164f7d0569a31113530` (2026-09-24).

## Impact

O-000023 does not collectively decide every explicit clause of P-000013, so
W-000032 cannot truthfully serve as complete-scope assurance evidence for that
Promise. G-000032's blanket characterization of all eight new Witnesses as
complete-scope evidence is also false for this lineage.

## Resolution

none

## References

- situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md
- situation/oracles/O-000023-judge-jwk-debug-redaction-complete-scope.md
- situation/witnesses/P-000013/W-000032-jwk-debug-redaction-complete-scope-pass.md
- situation/gaps/G-000032-complete-scope-pass-witnesses-outpace-frozen-promise-states.md
- situation/oracles/AGENTS.md
