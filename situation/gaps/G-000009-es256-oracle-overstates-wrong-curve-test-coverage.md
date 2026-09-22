# G-000009 — ES256 Oracle overstates wrong-curve test coverage

## State

open

## Gap

O-000002 credits `test_ecdsa_verification` with requiring wrong-curve and
invalid P-256 signatures to return an error, but the test's two wrong-curve
branches do not fail if verification unexpectedly succeeds. The invalid-signature
checks do require an error, so only the wrong-curve portion of F3 is unsupported
by the named executable decision.

## Relevance

O-000002 is marked implemented and its implementation-coverage table is the
contract for which ES256 legs are executable. W-000002 describes the retained
run as evidencing corresponding negative unit behavior, so the coverage and
witness language must not imply a decision the test does not make.

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

The implementation-coverage table and retrospective witness can overstate the
negative ES256 behavior actually decided by the retained run. A later assurance
judgment could incorrectly treat mismatched-curve rejection as executable
coverage unless the leg or test evidence is made honest.

## Resolution

none

## References

- `situation/oracles/O-000002-judge-es256-verification.md`
- `situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md`
- `src/core/jwk/tests.rs`
