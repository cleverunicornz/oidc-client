# W-000034 — Client-secret expiry sentinel-collision oracle re-observation passes

## Promise

situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md

## Oracle

situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md

## Result

PASS — the oracle's predeclared command battery, which W-000019 could not run
(BLOCKED: the host's offline crates.io index could not resolve the `base64`
dependency at head `226ef0be`), compiles and passes in full at this head. All
four Pass legs and all three Fail legs are decided by executed fixtures; the
oracle names no manual legs.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

The command is exactly the predeclared O-000017 command, run offline
(cargo/rustc 1.98.0, Linux) at the head above; result lines are verbatim. The
log is sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000034/cargo-test-client-secret-expiration-sentinel.log`
  retains `cargo test --offline --lib --features accept-rfc3339-timestamps
  client_secret_expiration_` (`exit=0`): `running 10 tests` ... `test result:
  ok. 10 passed; 0 failed; 0 ignored; 0 measured; 103 filtered out`. The
  selected tests are the complete `client_secret_expiration_` fixture set at
  this head — `..._epoch_serialization_rejected`,
  `..._epoch_rfc3339_rejected`, `..._expires_at_numeric_sweep`,
  `..._fractional_epoch_rejected`, `..._sub_second_serialization_rejected`,
  `..._expires_at_rfc3339_sweep`, `..._setter`, `..._absent`,
  `..._expires_at`, `..._never_expires` — matching the source enumeration of
  ten `client_secret_expiration_*` tests in `src/registration/tests.rs`
  (eight ungated at lines 649-886, two gated at lines 886-935).
- The dependency-resolution failure recorded by W-000019 did not recur:
  resolution and compilation completed offline at this head.
- The SHA-256 of the log is
  `6a5415ac33746d2c55306b3dc08c73c8f01349982ae134d4c7c5026f314aee00`,
  recorded in the adjacent `SHA256SUMS` and re-verified with `sha256sum -c`
  after writing.
- Non-claims: no live-provider run; cargo-deny and the full gate are deferred
  to PR CI. This witness observes one promise under one oracle at one head;
  promise state transitions remain with the closure corrector.

## Oracle legs

| Leg | Evidence |
|---|---|
| P1 | PASS — `test_client_secret_expiration_epoch_serialization_rejected` asserts a whole-second epoch `ExpiresAt` fails serialization with the sentinel-collision error and cannot produce numeric `0` (`src/registration/tests.rs` lines 837-852); `test_client_secret_expiration_sub_second_serialization_rejected` asserts a sub-second epoch value floors into the sentinel second and is refused at serialization (lines 853-870; guard at `src/registration/mod.rs` lines 785-790). |
| P2 | PASS — `test_client_secret_expiration_fractional_epoch_rejected` asserts `0.5` and `0.999` fail deserialization without entering either variant (lines 871-886; guard at `src/registration/mod.rs` lines 821-827). |
| P3 | PASS — `test_client_secret_expiration_epoch_rfc3339_rejected` asserts `"1970-01-01T00:00:00Z"` and `"1970-01-01T00:00:00.999Z"` fail deserialization under the feature (lines 887-904). |
| P4 | PASS — `test_client_secret_expiration_never_expires` asserts numeric `0` deserializes and serializes as the `NeverExpires` sentinel and round-trips (lines 648-680; serializer arm `src/registration/mod.rs` line 783). |
| F1 | PASS (negative executed) — the colliding direct `ExpiresAt` never serializes as numeric `0`; the P1 fixtures fail the run if the serializer ever succeeds (error path `src/registration/mod.rs` lines 785-790). |
| F2 | PASS (negative executed) — colliding numeric (P2 fixture) and RFC 3339 (P3 fixture) inputs never deserialize into either enum variant (rejection paths `src/registration/mod.rs` lines 811-827). |
| F3 | PASS (negative executed) — `NeverExpires` retains its numeric-`0` sentinel and variant identity through the round trip (P4 fixture). |
