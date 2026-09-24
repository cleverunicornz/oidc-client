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

situation/oracles/O-000011-judge-user-info-subject-binding.md

## State evidence

State `implemented` cites implementation commit
`78529bc948824f2dc3f6eb2270b2e9c5aa98876c`, which adds the bound tutorial
and example calls and the focused subject-binding fixture. The Promise,
Oracle, and Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/witnesses/P-000010/W-000013-user-info-subject-binding.md is a PASS
observation from the parent's final gate at
`b96b920f52e0d8b392edcf0b5d752fa570c2b356` and decides the named
O-000011 legs. The state remains `implemented`, not `assured`: O-000011 and
W-000013 do not decide the separately stated `None` expected-subject skip
behavior. That missing local coverage is visible as a closure repair
obligation, not evidence that the behavior fails.

## Residual

The tutorials are `no_run` documentation: compilation of the documented call is
exercised, not its execution against a live provider. The asynchronous tutorial
contains no user info call and therefore demonstrates no binding. The unbound
`None` expected-subject mode has no O-000011/W-000013 leg and remains
unassured until a dedicated local lineage decides it.

## References

- src/lib.rs (synchronous tutorial: bound `user_info` call and OIDC Core
  5.3.2 note)
- examples/gitlab.rs (bound `user_info` call)
- src/verification/tests.rs::test_user_info_subject_binding
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
