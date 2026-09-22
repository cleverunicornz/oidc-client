# G-000009 — ES256 wrong-curve coverage claim removed

## State

closed

## Gap

At the reviewed head, O-000002 credited `test_ecdsa_verification` with
requiring wrong-curve and invalid P-256 signatures to return an error, but the
test's wrong-curve branches did not fail on unexpected success. The
invalid-signature checks did require an error, so only that behavior was
decided by the named executable.

## Relevance

O-000002's implementation-coverage table needed to name only decisions the
test actually makes. W-000002's retrospective wording likewise could not
credit unsupported wrong-curve behavior.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `situation/oracles/O-000002-judge-es256-verification.md` F3 combines
  mismatched-curve and invalid-signature acceptance, and its F3 coverage row
  says `test_ecdsa_verification` requires both to return an error.
- In `src/core/jwk/tests.rs`, the P-256-key/P-384-algorithm branch at lines
  314–332 and the P-384-key/P-256-algorithm branch at lines 357–375 use
  `if let Some(err) = ... .err()`. If verification returns `Ok`, `.err()` is
  `None`, the body is skipped, and the test continues without failing.
- The `verify_invalid_signature` helper at lines 215–230 instead uses
  `expect_err`, so the invalid-signature half of F3 is executable.
- The test bytes are unchanged between witness head
  `24835e4b44caa8a0baae2ac5b865bbd6bdf355ba` and the reviewed head.

## Impact

At the reviewed head, the implementation-coverage table and retrospective
witness overstated the negative ES256 behavior actually decided by the
retained run. A later assurance judgment could have treated mismatched-curve
rejection as executable coverage.

## Resolution

closed — `9dd7999` makes mismatched-curve behavior outside P-000002's scope,
`4c32b62` limits O-000002 F3 to invalid-signature acceptance, and `7881bd7`
corrects W-000002's retained-run statement.

## References

- `situation/oracles/O-000002-judge-es256-verification.md`
- `situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md`
- `src/core/jwk/tests.rs`
