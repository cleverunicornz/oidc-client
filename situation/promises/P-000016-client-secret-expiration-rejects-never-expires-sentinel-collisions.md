# P-000016 — Client-secret expiry rejects never-expires sentinel collisions

## State

implemented

## Promise

`ClientSecretExpiration` never aliases an expiry to the numeric
never-expires sentinel. `ClientSecretExpiration::NeverExpires` is the only
value that serializes as numeric JSON `0`. A supplied
`ClientSecretExpiration::ExpiresAt` that resolves to the Unix epoch second,
including a subsecond value whose timestamp serialization rounds down to that
second, returns a serialization error instead of serializing as `0`. A numeric
fractional input or, with `accept-rfc3339-timestamps`, an RFC 3339 input that
resolves to the epoch second returns a deserialization error instead of
becoming `ExpiresAt` or `NeverExpires`.

## Scope

The manual `Serialize` and `Deserialize` implementations of
`ClientSecretExpiration` in `src/registration/mod.rs`, limited to the direct
and subsecond `ExpiresAt` values and numeric/RFC 3339 inputs that resolve to
the Unix epoch second. This promise does not change the accepted non-colliding
expiry values recorded by P-000014, the generic `helpers::Timestamp` adapter,
or timestamp semantics for any other claim.

## Oracle

situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md

## State evidence

State `implemented` cites correction commit
`f80568881fccd9e32ad99f593b39b6e54f9fa59a`, which adds the type-local
rejection branches and the focused epoch-collision tests in
`src/registration/tests.rs`. D-000018 selects this behavior. No witness has
been collected under this oracle yet.

## Residual

This promise does not assure timestamp inputs that do not resolve to the epoch
second, negative-date semantics, provider behavior beyond serde acceptance, or
any `helpers::Timestamp` consumer outside `ClientSecretExpiration`.

## References

- situation/decisions/D-000018-reject-client-secret-expiration-never-expires-sentinel-collisions.md
- situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md
- src/registration/mod.rs (`ClientSecretExpiration`)
- src/registration/tests.rs::test_client_secret_expiration_epoch_serialization_rejected
- src/registration/tests.rs::test_client_secret_expiration_sub_second_serialization_rejected
- src/registration/tests.rs::test_client_secret_expiration_fractional_epoch_rejected
- src/registration/tests.rs::test_client_secret_expiration_epoch_rfc3339_rejected
