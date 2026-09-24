# W-000029 — UserInfo subject-binding complete-scope observation passes

## Promise

situation/promises/P-000010-user-info-flows-bind-verified-id-token-subject.md

## Oracle

situation/oracles/O-000020-judge-user-info-subject-binding-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The retained
`user_info_subject_binding` battery now executes the bound JSON/signed
fixtures and the `none_mode` fixture (the `None` leg W-000022 left INVALID is
fixture-executed), the crate-root tutorial doctests compile, and the `gitlab`
example compiles while passing the verified ID-token subject. The `None`
plumbing leg (P6) and its negative (F2) are additionally decided by code
inspection named below.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000029/cargo-test-user-info-subject-binding.log`
  retains `cargo test --offline --lib --quiet -- user_info_subject_binding`
  (`exit=0`): `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 108
  filtered out`. The selected tests are `test_user_info_subject_binding` and
  `test_user_info_subject_binding_none_mode`. SHA-256
  `36db31419651151b89629382a817d7f5774d79a48032655fd429c57f83be1c11`.
- `situation/witnesses/evidence/W-000028/cargo-test-doc-reqwest-blocking.log`
  (same battery as W-000028; run once at this head) retains
  `cargo test --offline --doc --features reqwest-blocking` (`exit=0`):
  `test result: ok. 7 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out`.
  SHA-256
  `64a866971a6616f5f5f7429ead1d8005449d9f53a923b19b838fdcda975b64fc`.
- `situation/witnesses/evidence/W-000029/cargo-check-gitlab-example.log`
  retains `cargo check --offline --example gitlab --features reqwest-blocking`
  (`exit=0`): `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in
  1.32s` (the worktree prefix of the package-status line is sanitized to
  `[repository root]` per the G-000031 hygiene concern). SHA-256
  `21c25ab6750fc86907b4cf91abb2e8d200201cb6191d8394b536d988f4d733de`.
- Digests are recorded in the adjacent `SHA256SUMS` files and were re-verified
  with `sha256sum -c` after writing.
- Structural citations were inspected at this head; line numbers refer to the
  files at `d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba`.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_user_info_subject_binding` accepts a JSON response whose subject matches the supplied expected subject (`src/verification/tests.rs` lines 1727-1729). |
| P2 | PASS (negative executed) — a JSON response whose subject differs is rejected with `ClaimsVerificationError::InvalidSubject` (lines 1731-1738; raised at `src/user_info.rs` lines 365-379). |
| P3 | PASS — the same fixture accepts a signed response whose subject matches after signature verification (lines 1740-1761). |
| P4 | PASS (negative executed) — a signed response whose subject differs from the bound expected subject is rejected with `InvalidSubject` after successful signature verification (lines 1763-1774; raised at `src/verification/mod.rs` lines 1009-1022). |
| P5 | PASS — the crate-root tutorials compile with `reqwest-blocking` (doctest battery above), and the `gitlab` example compiles while passing the verified ID-token subject: `examples/gitlab.rs` lines 192-197 call `client.user_info(token, Some(id_token_claims.subject().clone()))`. |
| P6 | PASS — executed: `test_user_info_subject_binding_none_mode` accepts `sub: some_other_subject` through `from_json(..., None)` (lines 2157-2166) and through the signed verifier constructed with `None` after signature verification (lines 2168-2191). Structural: `Client::user_info` forwards the `Option<SubjectIdentifier>` untouched in both endpoint-state impls (`src/client.rs` lines 1367-1373, 1455-1467); `user_info_impl` passes it into both verifier constructors (`src/user_info.rs` lines 91, 108, 115); the verifier check iterates the Option so `None` skips comparison (`src/verification/mod.rs` lines 1009-1013: `self.expected_subject.iter().all(...)`), as does the JSON check (`src/user_info.rs` lines 365-368). |
| F1 | PASS (negative executed) — bound mismatches on both paths produce `InvalidSubject` and no other error variant (P2/P4 fixtures, exact-variant `match` arms). |
| F2 | PASS (negative executed and structural) — the `None` mode is accepted, not rejected (lines 2157-2191), and no hop replaces the caller's `None`: the value flows verbatim from `Client::user_info` through `user_info_impl` into the verifier field (lines 924/945 of `src/verification/mod.rs`) and into `from_json`'s parameter; the contrast arm of the fixture shows the same response rejected only when a bound subject is supplied (lines 2193-2204). |
| F3 | PASS — both documented calls retain their binding argument: the tutorials compile (doctest battery) and the `gitlab` example compiles with `Some(id_token_claims.subject().clone())` at `examples/gitlab.rs` lines 192-197. |
