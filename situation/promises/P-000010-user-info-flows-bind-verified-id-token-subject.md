# P-000010 — Documented UserInfo flows bind the verified ID-token subject

## State

implemented

## Promise

The documented user info flows bind the verified ID-token subject: the
crate-root synchronous tutorial and the `gitlab` example pass the verified
ID-token `sub` as the expected subject of the user info request, and a user
info response whose `sub` differs from the expected subject is rejected with
`ClaimsVerificationError::InvalidSubject` on both the JSON and the signed JWT
response path. The general `Client::user_info` API keeps
`Option<SubjectIdentifier>`: passing `None` legitimately skips the subject
check for callers without a verified ID token. Per OpenID Connect Core
Section 5.3.2, the optional `at_hash` check does not substitute for this
subject binding.

## Scope

The subject-binding behavior observable by consumers of the documented flows:
the crate-root tutorial documents and demonstrates the bound call, the gitlab
example performs it, and the verifier enforces it on both response encodings.
The unbound (`None`) mode's skip behavior is part of the stated contract
(documented at `Client::user_info`). This promise does not cover nonce,
issuer, audience, or signature verification of the user info response (see
P-000008), nor flows outside the documented surfaces.

## Oracle

situation/oracles/O-000020-judge-user-info-subject-binding-complete-scope.md

## State evidence

State `implemented` cites implementation commit
`78529bc948824f2dc3f6eb2270b2e9c5aa98876c`, which adds the bound tutorial
and example calls and the focused subject-binding fixture. The Promise,
Oracle, and Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/oracles/O-000020-judge-user-info-subject-binding-complete-scope.md
supersedes O-000011 for this Promise's complete Scope. Its first successor
observation,
situation/witnesses/P-000010/W-000022-user-info-subject-binding-complete-scope-incomplete.md,
is INVALID: it preserves the absence of independent `None` expected-subject
and documentation-leg evidence. The state therefore remains `implemented`,
not `assured`. W-000013 remains the PASS observation of the historical,
narrower O-000011 rule.

## Residual

No assurance is claimed until O-000020 has a valid witness for every declared
Scope clause. W-000022 makes the `None` expected-subject and documentation
compile legs visible; that evidence boundary does not narrow this Promise.
The tutorials remain `no_run` documentation, and the asynchronous tutorial
contains no user info call.

## References

- src/lib.rs (synchronous tutorial: bound `user_info` call and OIDC Core
  5.3.2 note)
- examples/gitlab.rs (bound `user_info` call)
- src/verification/tests.rs::test_user_info_subject_binding
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
