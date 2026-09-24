# W-000022 — UserInfo subject-binding complete-scope observation is incomplete

## Promise

situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md

## Oracle

situation/oracles/O-000020-judge-user-info-subject-binding-complete-scope.md

## Result

INVALID — the fresh bounded command passed the matching and mismatching JSON
and signed-subject fixtures, but it did not execute the documentation compile
leg or independently observe the `None` expected-subject mode. This is an
incomplete observation of the complete-scope rule, not a partial PASS and not
assurance.

## Head

2228a05f4f53b9304c0509b978c2aff7a8fedb12

## Observed

2026-09-24

## Evidence

- `situation/witnesses/evidence/W-000022/cargo-test-user-info-subject-binding.log`
  retains the sanitized output of `cargo test --offline --lib --quiet --
  user_info_subject_binding` at this head (`exit=0`). Its SHA-256 is
  `fdf9548a68618b6b21c9b6ae6837a40d59ef267021be9f2622dcc7371e3aaadc`
  in the adjacent `SHA256SUMS` file.
- The command selects only `test_user_info_subject_binding`; it cannot decide
  the newly explicit `None` leg or the documented compile legs listed as
  `INVALID` below.
- W-000013 remains the PASS observation of superseded O-000011, not of this
  successor rule.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — the matching JSON subject assertion passed in the retained command. |
| P2 | PASS — the mismatching JSON subject `InvalidSubject` assertion passed in the retained command. |
| P3 | PASS — the matching signed subject assertion passed in the retained command. |
| P4 | PASS — the mismatching signed subject `InvalidSubject` assertion passed in the retained command. |
| P5 | INVALID — the command did not execute the tutorial or `gitlab` compile legs. |
| P6 | INVALID — no `None` expected-subject observation was retained for this successor rule. |
