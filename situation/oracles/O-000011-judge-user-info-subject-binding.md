# O-000011 — Judge UserInfo subject binding in documented flows

## State

implemented

## Judges

situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md

## Inputs

The scoped Cargo test command below over the JSON and RS256 signed fixtures in
`src/verification/tests.rs`, the crate-root tutorial doctests in `src/lib.rs`,
and the `gitlab` example target.

## Pass

- P1: A JSON user info response whose `sub` matches the expected subject is
  accepted.
- P2: A JSON user info response whose `sub` differs from the expected subject
  is rejected with `ClaimsVerificationError::InvalidSubject`.
- P3: A signed user info JWT whose `sub` matches the expected subject is
  accepted.
- P4: A signed user info JWT whose `sub` differs from the expected subject is
  rejected with `ClaimsVerificationError::InvalidSubject` after signature
  verification succeeds.
- P5: The crate-root synchronous tutorial and the `gitlab` example compile
  with the verified ID-token subject passed as the expected subject of the
  user info request.

## Fail

- F1: A mismatched-`sub` response is accepted or rejected with any error other
  than `InvalidSubject` on either path.

## Implementation

`cargo test --offline --lib -- verification::tests::test_user_info_subject_binding`
executes the P1–P4 and F1 legs. `cargo test --offline --doc --features
reqwest-blocking` and `cargo check --offline --example gitlab --features
reqwest-blocking` execute the P5 leg.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Matching JSON subject accepted. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P2 | Mismatched JSON subject rejected with InvalidSubject. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P3 | Matching signed subject accepted. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P4 | Mismatched signed subject rejected after signature verification. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P5 | Documented flows compile with the bound subject. | `cargo test --offline --doc --features reqwest-blocking` |
| F1 | Mismatch yields InvalidSubject and nothing else. | `src/verification/tests.rs::test_user_info_subject_binding` |
