# O-000015 — Judge registration client-secret expiration semantics

## State

implemented

## Judges

situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md

## Inputs

Deserialization and serialization of `CoreClientRegistrationResponse`
values whose `client_secret_expires_at` is the numeric `0`, a real
timestamp (`1526545306`), or absent; the values returned by
`client_secret_expires_at()` and stored by
`set_client_secret_expires_at`; and the serialized JSON forms. Evidence
is produced by the focused command
`cargo test --offline --lib registration` on the judged head, plus
`cargo check --offline --tests` for the compile-only leg.

## Pass

- P1 — Numeric `0` deserializes to
  `Some(ClientSecretExpiration::NeverExpires)` and never to an epoch
  `ExpiresAt` value or an error.
- P2 — A real timestamp (`1526545306`) deserializes to
  `Some(ClientSecretExpiration::ExpiresAt(...))` carrying
  `Utc.timestamp_opt(1526545306, 0)` (floor-rounding semantics preserved).
- P3 — An absent field deserializes to `None` and serializes with the
  field omitted.
- P4 — Round trips are stable: `NeverExpires` →
  `"client_secret_expires_at":0` → `NeverExpires`; `ExpiresAt` of
  `1526545306` → `"client_secret_expires_at":1526545306` → the same
  `ExpiresAt`; absent → omitted → `None`.
- P5 — `set_client_secret_expires_at` accepts both states; a stored
  `NeverExpires` reads back as `Some(&ClientSecretExpiration::NeverExpires)`,
  not as an expired timestamp, and serializes to numeric `0`.
- P6 — Non-regression: the pre-existing registration tests (including
  `test_response_serialization` with its migrated real-timestamp
  expectation and the wave-2 bearer/Content-Type lanes) pass unchanged.
- P7 — The `#[ignore]`d live test `tests/rp_certification_dynamic.rs`
  still compiles: `cargo check --offline --tests` succeeds.

## Fail

Any of: numeric `0` deserializing to an `ExpiresAt` (epoch) value or an
error; a real timestamp deserializing to `NeverExpires`, `None`, or the
wrong instant; an absent field deserializing to anything but `None` or
serializing the field; any round trip losing the state; a stored
`NeverExpires` reading back as expired; `NeverExpires` serializing to any
form other than numeric `0`; any pre-existing registration test failing;
or `cargo check --offline --tests` failing.

## Implementation

The executable checks are the four focused expiration tests plus the
pre-existing registration tests in `src/registration/tests.rs`, run by
`cargo test --offline --lib registration`; the compile leg runs
`cargo check --offline --tests`.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Numeric `0` → `NeverExpires`, serialized back to `0` | `src/registration/tests.rs::test_client_secret_expiration_never_expires` |
| P2 | Real timestamp → matching `ExpiresAt` | `src/registration/tests.rs::test_client_secret_expiration_expires_at`; the migrated real-timestamp assertion in `::test_response_serialization` |
| P3 | Absent → `None`, omitted on serialization | `src/registration/tests.rs::test_client_secret_expiration_absent` |
| P4 | Round-trip stability for all three states | the round-trip halves of the three tests above; the response round trip in `::test_response_serialization` |
| P5 | Setter stores both states; `NeverExpires` never reads as expired | `src/registration/tests.rs::test_client_secret_expiration_setter` |
| P6 | Pre-existing registration tests pass unchanged | `test_response_serialization`, `test_metadata_serialization`, `test_metadata_serialization_minimal`, and the wave-2 lanes (`test_registration_*`) run by the focused command |
| P7 | Ignored live test compiles | `cargo check --offline --tests` |

## Rules note

Witnesses do not exist yet; they are to be collected under
`situation/witnesses/P-000014/` by the parent's final gate. This oracle
does not record results.
