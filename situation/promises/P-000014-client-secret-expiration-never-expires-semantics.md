# P-000014 — Registration client-secret expiry exposes explicit never/expires-at/absent semantics

## State

assured

## Promise

A consumer of a dynamic client registration response
(`ClientRegistrationResponse::client_secret_expires_at()`) observes exactly
three distinct states for `client_secret_expires_at`:

- `None` — the field is absent from the response;
- `Some(ClientSecretExpiration::NeverExpires)` — the JSON numeric value is
  `0`, which OpenID Connect Dynamic Client Registration 2 defines as "the
  client secret DOES NOT expire"; it is never surfaced as an
  already-expired 1970-01-01T00:00:00Z timestamp;
- `Some(ClientSecretExpiration::ExpiresAt(datetime))` — a numeric timestamp
  that resolves to a Unix epoch second other than `0`, carrying the shared
  `Timestamp` adapter's round-down-to-second semantics (and RFC 3339 string
  acceptance under the `accept-rfc3339-timestamps` feature).

Serialization is round-trip stable for those three representable states:
`NeverExpires` serializes to numeric `0`, a non-colliding `ExpiresAt`
serializes to its timestamp in seconds, and an absent field stays omitted.
The setter (`set_client_secret_expires_at`) accepts both representable states,
and a stored `NeverExpires` value is never read back as an expired timestamp.
Values or inputs that resolve to the epoch second are deliberately outside this
Promise's three-state assurance and are rejected under P-000016 instead of
aliasing the sentinel. The shared `helpers::Timestamp` adapter used by other
claims (`client_id_issued_at`, ID-token `exp`/`iat`/`auth_time`) is unchanged.

## Scope

The `ClientSecretExpiration` type and the `client_secret_expires_at` field,
getter, and setter of `ClientRegistrationResponse` in
`src/registration/mod.rs`, limited to the three representable states described
above. P-000016 owns epoch-collision rejection. Outside scope:
`client_id_issued_at` and every other consumer of `helpers::Timestamp`,
token-lifetime expiry semantics elsewhere in the crate, and provider-side
interpretation of the sentinel.

## Oracle

situation/oracles/O-000015-judge-client-secret-expiration-semantics.md

## State evidence

State `assured` cites
situation/oracles/O-000015-judge-client-secret-expiration-semantics.md and
the PASS witness
situation/witnesses/P-000014/W-000017-client-secret-expiration-semantics.md
at `b96b920f52e0d8b392edcf0b5d752fa570c2b356`, which decides O-000015's
three-state legs.

Implementation commit `f93c8c79055fa5caf5adb6690d9dec123974b702` introduced
the enum, migrated the field/getter/setter, and added the original focused
tests. The Promise, Oracle, and Witness records were attached
retrospectively in `9c4013a9d93a93e65084b3475022c43059e64fc2`. The later
epoch-collision correction at
`f80568881fccd9e32ad99f593b39b6e54f9fa59a` is a separate P-000016/D-000018
lineage and does not enlarge this Promise's assurance scope.

## Residual

Assurance is complete only for the three representable states decided by
O-000015/W-000017. The witness exercised no live-network flow — the
`#[ignore]`d certification tests compiled but never ran — so live-provider
behavior (how real providers send or interpret the `0` sentinel) remains
unobserved. Epoch-collision inputs and values are rejected under P-000016,
rather than being assured here. This promise does not cover wire forms other
than those `helpers::Timestamp` accepts, provider meaning beyond the
registration field, or `client_id_issued_at`.

## References

- situation/oracles/O-000015-judge-client-secret-expiration-semantics.md
- situation/decisions/D-000017-prepublication-hardening-maintenance-delta-registration-expiry-surface.md
- situation/decisions/D-000018-reject-client-secret-expiration-never-expires-sentinel-collisions.md
- situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md
- https://openid.net/specs/openid-connect-registration-1_0.html#ClientMetadata
- src/registration/mod.rs (`ClientSecretExpiration`)
- src/registration/tests.rs::test_client_secret_expiration_never_expires
- src/registration/tests.rs::test_client_secret_expiration_expires_at
- src/registration/tests.rs::test_client_secret_expiration_absent
- src/registration/tests.rs::test_client_secret_expiration_setter
