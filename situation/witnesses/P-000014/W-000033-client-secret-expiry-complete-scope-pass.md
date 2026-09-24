# W-000033 — Registration client-secret expiry complete-scope observation passes

## Promise

situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md

## Oracle

situation/oracles/O-000024-judge-client-secret-expiration-complete-scope.md

## Result

PASS — every oracle leg is decided at this head. The two registration
batteries execute 16 tests without the feature and 18 with
`accept-rfc3339-timestamps`, including the generic numeric sweep and — new
relative to W-000026 — the feature-gated RFC3339 sweep, so the generic
non-colliding acceptance legs W-000026 left INVALID are fixture-executed. The
generic adapter domain (P3's full extent and P4's delegation) is bounded
honestly below by inspection of the sweep helper and the `Timestamp` adapter:
numeric magnitudes beyond the representable UTC range do not resolve to an
epoch second and error; that error boundary is the adapter's declared domain,
not a promise violation.

## Head

d35d70a7e570ea1cbf9e0abf8f34d61f3035a1ba

## Observed

2026-09-24

## Evidence

All commands ran offline (cargo/rustc 1.98.0, Linux) at the head above; result
lines are verbatim. Logs are sanitized (no machine-local paths) and digested:

- `situation/witnesses/evidence/W-000030/cargo-test-registration.log` (same
  battery as W-000030; run once at this head) retains
  `cargo test --offline --lib --quiet -- registration::` (`exit=0`):
  `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 94 filtered
  out`. SHA-256
  `76dce5568d13b6f486e231e2f52ef91a52f0579226a800a60d8adb16aa9ea0de`.
- `situation/witnesses/evidence/W-000033/cargo-test-registration-feature.log`
  retains `cargo test --offline --lib --quiet --features
  accept-rfc3339-timestamps -- registration::` (`exit=0`): `test result: ok.
  18 passed; 0 failed; 0 ignored; 0 measured; 95 filtered out`. The two
  additional tests are `test_client_secret_expiration_epoch_rfc3339_rejected`
  and `test_client_secret_expiration_expires_at_rfc3339_sweep`. SHA-256
  `26a5f09594a8e2ad78f8353ea266709558891fd27b147a101000f93f126f302a`.
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
| P1 | PASS — `test_client_secret_expiration_never_expires` deserializes numeric `0` to `NeverExpires` and serializes it back as numeric `0` (`src/registration/tests.rs` lines 648-680; serializer arm `serialize_i64(0)` at `src/registration/mod.rs` line 783, deserializer sentinel match at lines 806-813). |
| P2 | PASS — `test_client_secret_expiration_absent` deserializes the absent field to `None` and serializes it as omitted (`src/registration/tests.rs` lines 763-786; `Option<ClientSecretExpiration>` with `skip_serializing_none` at `src/registration/mod.rs` lines 834, 857). |
| P3 | PASS — fixture plus bounded generic decision. Executed: `test_client_secret_expiration_expires_at` round-trips a concrete expiry (lines 681-714) and `test_client_secret_expiration_expires_at_numeric_sweep` sweeps the generic numeric domain — pre-1970 negative seconds, fractional floors away from and toward zero, the smallest non-colliding second, a recent 10-digit second, and far-future i64 seconds (`src/registration/tests.rs` lines 748-761). The sweep helper asserts each case deserializes as `ExpiresAt` at the expected floor-to-second and serializes back to exactly that second (`assert_expires_at_cases`, lines 720-742). Generic domain, decided by inspection: resolution is `Timestamp::to_utc` (`src/helpers.rs` lines 423-437) — integral i64 seconds pass through; non-integral values floor (`secs_f64.floor()`, line 430) and anything that `Utc.timestamp_opt(secs, nsecs).single()` cannot represent errors (line 436 `ok_or(())`), which the sweep test documents as "numeric magnitudes beyond the representable UTC range do not resolve to an epoch second and are outside this domain" (lines 744-747). The serializer rounds down via `Timestamp::from_utc` (lines 419-421). This representable bound is recorded as the adapter's domain, not a gap in the promise. |
| P4 | PASS — feature-gated acceptance is fixture-executed: `test_client_secret_expiration_expires_at_rfc3339_sweep` sweeps non-colliding RFC3339 inputs (smallest non-colliding second, fractional pre-epoch, pre-1970, and further cases, `src/registration/tests.rs` lines 905-935) through the same helper, proving deserialization to `ExpiresAt` and serialization back to epoch seconds. Structural delegation: `ClientSecretExpiration::deserialize` forwards all non-sentinel input to `timestamp.to_utc()` (`src/registration/mod.rs` lines 815-828), whose `Rfc3339` arm parses with `DateTime::parse_from_rfc3339` (`src/helpers.rs` lines 438-442); serialization returns through `Timestamp::from_utc(...).serialize` (`src/registration/mod.rs` line 791), i.e. epoch seconds. |
| P5 | PASS — the absent (P2), never-expires (P1), and expires-at (P3/P4) fixtures round trip without aliasing; the serializer/deserializer arms are disjoint by construction (`src/registration/mod.rs` lines 782-793, 806-828), and the collision rejections below keep the sentinel exclusive. |
| P6 | PASS — `test_client_secret_expiration_setter` sets both representable states via `set_client_secret_expires_at`, asserts the getter preserves each, and asserts `NeverExpires` serializes as numeric `0` (`src/registration/tests.rs` lines 788-836; setter at `src/registration/mod.rs` line 925). |
| F1 | PASS (negative executed) — numeric `0` never becomes an epoch `ExpiresAt` (P1 fixture) and `ExpiresAt` values resolving to the epoch second cannot serialize: `test_client_secret_expiration_epoch_serialization_rejected` (lines 837-852; serializer guard `src/registration/mod.rs` lines 785-790). |
| F2 | PASS (negative executed) — the absent field stays absent (P2 fixture). |
| F3 | PASS (negative executed) — non-colliding numeric input deserializes distinctly with floor-to-second semantics and round-trips exactly (P3 sweep); fractional inputs resolving to the sentinel second are rejected rather than entering either variant (`test_client_secret_expiration_fractional_epoch_rejected`, lines 871-886). |
| F4 | PASS (negative executed) — feature-gated non-colliding RFC3339 input deserializes as `ExpiresAt` and serializes as its epoch seconds (P4 sweep); colliding RFC3339 epoch inputs, the documented boundary outside this promise, are rejected (`test_client_secret_expiration_epoch_rfc3339_rejected`, lines 887-904; deserializer guard `src/registration/mod.rs` lines 821-827). |
| F5 | PASS (negative executed) — the P5/P6 fixtures fail on any lost state or setter mutation; all pass. |
