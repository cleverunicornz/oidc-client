# O-000020 — Judge UserInfo subject binding complete scope

## State

implemented

## Judges

situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md

## Inputs

At the judged head, JSON and signed UserInfo claims with matching and
mismatching subjects, the crate-root tutorial and `gitlab` example calls, and
the `Client::user_info` path carrying an optional expected subject. This
successor retains O-000011's bound-response fixture and adds the previously
unjudged `None` expected-subject mode.

## Pass

- P1 — A JSON UserInfo response whose subject matches the supplied expected
  subject is accepted.
- P2 — A JSON UserInfo response whose subject differs from the supplied
  expected subject is rejected with `ClaimsVerificationError::InvalidSubject`.
- P3 — A signed UserInfo response whose subject matches the supplied expected
  subject is accepted after signature verification.
- P4 — A signed UserInfo response whose subject differs from the supplied
  expected subject is rejected with `ClaimsVerificationError::InvalidSubject`
  after signature verification.
- P5 — The synchronous crate-root tutorial and `gitlab` example compile while
  passing the verified ID-token subject as the expected user-info subject.
- P6 — `Client::user_info(..., None)` preserves the absent expected subject;
  JSON and signed UserInfo paths accept any response subject rather than
  applying a comparison intended for a verified ID token.

## Fail

- F1 — A mismatching JSON or signed subject is accepted, or receives an error
  other than `InvalidSubject` after the applicable signature verification.
- F2 — A `None` expected subject is rejected merely because it has no value,
  or is replaced with a subject value that changes the caller's requested mode.
- F3 — A documented synchronous call or `gitlab` example stops passing the
  verified ID-token subject to `user_info`.

## Implementation

`cargo test --offline --lib --quiet -- user_info_subject_binding` executes the
bound JSON and signed fixtures. The documentation compile and `None` mode use
the coverage entries below until their own observations are retained.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Matching JSON subject is accepted. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P2 | Mismatched JSON subject gets `InvalidSubject`. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P3 | Matching signed subject is accepted. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P4 | Mismatched signed subject gets `InvalidSubject`. | `src/verification/tests.rs::test_user_info_subject_binding` |
| P5 | Documented flows compile with the verified subject. | `cargo test --offline --doc --features reqwest-blocking`; `cargo check --offline --example gitlab --features reqwest-blocking` |
| P6 | `None` reaches both subject checks and skips comparison. | manual — `src/client.rs::Client::user_info`, `src/user_info.rs::Client::user_info_impl`, `UserInfoClaims::from_json`, and `UserInfoVerifier::verified_claims` |
| F1 | Bound mismatches do not pass. | `src/verification/tests.rs::test_user_info_subject_binding` |
| F2 | Absent subject stays an intentional skip. | manual — the Option-iteration checks in `src/user_info.rs` and `src/verification/mod.rs` |
| F3 | Documented calls retain their binding argument. | `cargo test --offline --doc --features reqwest-blocking`; `cargo check --offline --example gitlab --features reqwest-blocking` |

## References

- Supersedes O-000011 for P-000010's complete declared scope; W-000013
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000010/W-000022-user-info-subject-binding-complete-scope-incomplete.md
