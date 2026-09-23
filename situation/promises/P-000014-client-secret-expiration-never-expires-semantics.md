# P-000014 — Registration client-secret expiry exposes explicit never/expires-at/absent semantics

## State

implemented

## Promise

A consumer of a dynamic client registration response
(`ClientRegistrationResponse::client_secret_expires_at()`) observes exactly
three distinct states for `client_secret_expires_at`:

- `None` — the field is absent from the response;
- `Some(ClientSecretExpiration::NeverExpires)` — the JSON numeric value is
  `0`, which OpenID Connect Dynamic Client Registration 2 defines as "the
  client secret DOES NOT expire"; it is never surfaced as an
  already-expired 1970-01-01T00:00:00Z timestamp;
- `Some(ClientSecretExpiration::ExpiresAt(datetime))` — any other numeric
  timestamp, carrying the shared `Timestamp` adapter's
  round-down-to-second semantics (and RFC 3339 string acceptance under the
  `accept-rfc3339-timestamps` feature).

Serialization is round-trip stable: `NeverExpires` serializes to numeric
`0`, `ExpiresAt` serializes to its timestamp in seconds, and an absent
field stays omitted. The setter (`set_client_secret_expires_at`) accepts
both states, and a stored `NeverExpires` value is never read back as an
expired timestamp. The shared `helpers::Timestamp` adapter used by other
claims (`client_id_issued_at`, ID-token `exp`/`iat`/`auth_time`) is
unchanged.

## Scope

The `ClientSecretExpiration` type and the `client_secret_expires_at`
field, getter, and setter of `ClientRegistrationResponse` in
`src/registration/mod.rs`, including their serde behavior. Outside scope:
`client_id_issued_at` and every other consumer of `helpers::Timestamp`,
token-lifetime expiry semantics elsewhere in the crate, and
provider-side interpretation of the sentinel.

## Oracle

situation/oracles/O-000015-judge-client-secret-expiration-semantics.md

## State evidence

State `implemented` cites the Stream D implementing commit of the
pre-publication hardening batch on branch `fix/prepublication-hardening`
(code and records land in the same commit). It introduces the
`ClientSecretExpiration` enum with type-scoped serde, migrates the field,
getter, setter, and test expectations, and adds four focused tests to
`src/registration/tests.rs`. Focused results at that state:
`cargo test --offline --lib registration` → 11 passed, 0 failed;
`cargo check --offline --tests` clean (including the `#[ignore]`d
`tests/rp_certification_dynamic.rs`, which compiles unchanged through the
getter and derived `Debug`); the four new tests also pass under
`--features accept-rfc3339-timestamps`. A falsification run with the
zero-sentinel check disabled failed exactly
`test_client_secret_expiration_never_expires`. No Witness exists yet:
witnesses are to be collected under `situation/witnesses/P-000014/` by
the parent's final gate.

## Residual

Assurance is not complete: the oracle has not been applied to a witness,
so this promise is not `assured` and is not yet invariant behavior under
the supersession rule. The promise does not cover wire forms other than
those `helpers::Timestamp` already accepts (numeric seconds; RFC 3339
strings under the feature), does not constrain what a provider means by
`0`, and does not cover `client_id_issued_at`.

## References

- situation/oracles/O-000015-judge-client-secret-expiration-semantics.md
- situation/decisions/D-000017-prepublication-hardening-maintenance-delta-registration-expiry-surface.md
- https://openid.net/specs/openid-connect-registration-1_0.html#ClientMetadata
- src/registration/mod.rs (`ClientSecretExpiration`)
- src/registration/tests.rs::test_client_secret_expiration_never_expires
- src/registration/tests.rs::test_client_secret_expiration_expires_at
- src/registration/tests.rs::test_client_secret_expiration_absent
- src/registration/tests.rs::test_client_secret_expiration_setter
