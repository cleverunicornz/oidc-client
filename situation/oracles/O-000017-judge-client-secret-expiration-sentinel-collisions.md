# O-000017 — Judge client-secret expiry never-expires sentinel collisions

## State

implemented

## Judges

situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md

## Inputs

The `ClientSecretExpiration` serializer and deserializer at the judged head,
and the focused registration tests for exact-epoch and subsecond `ExpiresAt`
values, fractional numeric input, feature-gated RFC 3339 input, and the
ordinary `NeverExpires` round trip. The executable observation runs:

`cargo test --offline --lib --features accept-rfc3339-timestamps client_secret_expiration_`

## Pass

- P1: Exact-epoch and subsecond `ExpiresAt` values return serialization errors
  and cannot produce numeric JSON `0`.
- P2: Fractional numeric inputs that resolve to the epoch second return
  deserialization errors and do not become either enum variant.
- P3: With `accept-rfc3339-timestamps`, exact and subsecond RFC 3339 epoch
  inputs return deserialization errors.
- P4: `NeverExpires` still serializes to numeric `0` and round-trips as
  `NeverExpires`.

## Fail

- F1: A colliding direct `ExpiresAt` serializes successfully as numeric `0`.
- F2: A colliding numeric or RFC 3339 input deserializes successfully as
  `ExpiresAt` or `NeverExpires`.
- F3: `NeverExpires` fails to serialize as numeric `0` or to round-trip as
  `NeverExpires`.

## Implementation

`cargo test --offline --lib --features accept-rfc3339-timestamps client_secret_expiration_`
executes the focused registration tests.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Exact and subsecond direct values cannot serialize into the sentinel. | `src/registration/tests.rs::test_client_secret_expiration_epoch_serialization_rejected`; `::test_client_secret_expiration_sub_second_serialization_rejected` |
| P2 | Fractional numeric values resolving to epoch are rejected. | `src/registration/tests.rs::test_client_secret_expiration_fractional_epoch_rejected` |
| P3 | Feature-gated RFC 3339 epoch values are rejected. | `src/registration/tests.rs::test_client_secret_expiration_epoch_rfc3339_rejected` |
| P4 | The exclusive `NeverExpires` sentinel still round-trips. | `src/registration/tests.rs::test_client_secret_expiration_never_expires` |
| F1 | Colliding direct `ExpiresAt` never succeeds as numeric `0`. | the P1 tests |
| F2 | Colliding wire inputs never enter either enum variant. | the P2 and P3 tests |
| F3 | `NeverExpires` retains its numeric sentinel and variant identity. | the P4 test |
