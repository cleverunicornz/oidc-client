# G-000008 — Baseline parity evidence corrected for test deltas

## State

closed

## Gap

At the reviewed head, O-000004 and W-000001 described the imported upstream
offline tests as unchanged except for crate-name import renames, while the
witnessed tree also contained classified mechanical-lint and allow-addition
deltas. W-000001 additionally cited an absent exhaustive classification.

## Relevance

P-000001's imported-baseline lineage relies on O-000004 for its current
judgment rule and retains W-000001 as the historical O-000001 observation.
The test run itself was real, but the unchanged-suite premise needed correction
without rewriting the historical Oracle.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md` calls the
  offline suite unchanged in both Inputs and Pass leg P6, allowing only required
  crate-name import renames.
- `situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md`
  marks P6 PASS and says D-000004 records the only allowed crate-name test-import
  changes.
- `situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`
  records a `CoreIdTokenVerifier<'_>` change in
  `tests/rp_certification_code.rs` and a justified `#[allow(dead_code)]` plus
  explanatory comment in `tests/rp_common.rs`, in addition to import renames.
  The same Decision says the exhaustive classification lands in W-000001, but
  that classification was removed from the witness during this closure.
- The repository diff from import commit
  `eeb0e6d847d7665cc8b0c9e46b9716323ded3c6a` to witness head
  `24835e4b44caa8a0baae2ac5b865bbd6bdf355ba` shows both non-rename test
  changes.

## Impact

At the reviewed head, the record set could credit a historical or future parity
leg using a premise its fidelity Decision contradicted. The uncertainty was the
narrower unchanged-test claim and absent classification, not whether the test
run happened.

## Resolution

closed — `eb4e706` corrects D-000004's fidelity and promotion links,
`973c03b` corrects P-000001's evidence boundary, `d5b09f0` makes O-000004 P6
execution-only, and `d7b1257` preserves W-000001 as an honest historical
O-000001 observation.

## References

- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
- `situation/oracles/O-000001-judge-full-oidc-rp-flow-parity-with-upstream.md`
- `situation/oracles/O-000004-judge-imported-oidc-rp-baseline.md`
- `situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md`
- `situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`
