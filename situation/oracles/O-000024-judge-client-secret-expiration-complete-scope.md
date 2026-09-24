# O-000024 — Judge registration client-secret expiry complete scope

## State

implemented

## Judges

situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md

## Inputs

At the judged head, `ClientSecretExpiration` serialization and deserialization,
the registration response field/getter/setter, and numeric and feature-gated
RFC3339 timestamp inputs. This successor retains O-000015's concrete
three-state fixtures and adds the previously omitted generic non-colliding
timestamp and feature-gated RFC3339 acceptance legs for the current
serializer/deserializer.

## Pass

- P1 — Numeric JSON `0` deserializes to `NeverExpires`, never to an epoch
  `ExpiresAt` or an error, and serializes back as numeric `0`.
- P2 — An absent field deserializes to `None` and serializes as omitted.
- P3 — Every numeric timestamp that resolves to a Unix epoch second other than
  `0` deserializes as `ExpiresAt` with the shared `Timestamp` floor-to-second
  semantics and serializes back to that second.
- P4 — With `accept-rfc3339-timestamps`, every valid non-colliding RFC3339
  timestamp accepted by `Timestamp` deserializes as `ExpiresAt` and serializes
  back to epoch seconds.
- P5 — The absent, never-expires, and non-colliding expires-at states round
  trip without aliasing one another.
- P6 — The setter accepts both representable enum states, preserves their
  getter result, and serializes `NeverExpires` as numeric `0`.

## Fail

- F1 — Numeric `0` becomes an epoch `ExpiresAt`, errors, or serializes in any
  form other than numeric `0` for `NeverExpires`.
- F2 — An absent field becomes a value or serializes as present.
- F3 — A non-colliding numeric timestamp errors, aliases another state, loses
  floor-to-second semantics, or serializes as a different second.
- F4 — A valid feature-gated non-colliding RFC3339 timestamp errors, aliases
  another state, or does not serialize as its epoch seconds.
- F5 — A representable three-state round trip loses its state, or the setter
  changes a stored value.

## Implementation

`cargo test --offline --lib --quiet --features accept-rfc3339-timestamps
client_secret_expiration_` executes the retained concrete expiration and
collision fixtures. Generic non-colliding numeric/RFC3339 acceptance requires
the coverage entries below until its own observations are retained.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Numeric sentinel stays `NeverExpires`. | `src/registration/tests.rs::test_client_secret_expiration_never_expires` |
| P2 | Absent stays omitted. | `src/registration/tests.rs::test_client_secret_expiration_absent` |
| P3 | Non-colliding numeric timestamps retain adapter semantics. | `src/registration/tests.rs::test_client_secret_expiration_expires_at`; manual for the generic `Timestamp::to_utc` domain |
| P4 | Feature-gated non-colliding RFC3339 timestamps are accepted. | manual — `src/registration/mod.rs::ClientSecretExpiration::deserialize` delegates non-sentinel input to feature-gated `Timestamp::to_utc` |
| P5 | Three states round trip without aliasing. | `test_client_secret_expiration_never_expires`; `test_client_secret_expiration_expires_at`; `test_client_secret_expiration_absent` |
| P6 | Setter preserves representable states. | `src/registration/tests.rs::test_client_secret_expiration_setter` |
| F1 | Sentinel never aliases an epoch expiry. | P1 fixture; `test_client_secret_expiration_epoch_serialization_rejected` |
| F2 | Absent never becomes present. | P2 fixture |
| F3 | Numeric non-colliding input remains distinct. | P3 fixture; manual generic adapter decision |
| F4 | Feature-gated RFC3339 input remains distinct. | manual feature-gated adapter decision; `test_client_secret_expiration_epoch_rfc3339_rejected` bounds collision behavior outside this Promise |
| F5 | Round trips and setter retain identity. | P5/P6 fixtures |

## References

- Supersedes O-000015 for P-000014's complete declared scope; W-000017
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000014/W-000026-client-secret-expiry-complete-scope-incomplete.md
- situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md
