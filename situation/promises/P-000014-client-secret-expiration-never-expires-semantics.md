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

situation/oracles/O-000024-judge-client-secret-expiration-complete-scope.md

## State evidence

State `implemented` cites implementation commit
`f93c8c79055fa5caf5adb6690d9dec123974b702`, which introduced the enum,
migrated the field/getter/setter, and added the original focused tests. The
Promise, original Oracle, and original Witness records were attached
retrospectively in `9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/oracles/O-000024-judge-client-secret-expiration-complete-scope.md
supersedes O-000015 for this Promise's complete Scope. Its first successor
observation,
situation/witnesses/P-000014/W-000026-client-secret-expiry-complete-scope-incomplete.md,
is INVALID: it preserves the absence of generic non-colliding numeric,
feature-gated RFC3339-acceptance, and shared-adapter-isolation evidence at the
current serializer/deserializer. The state is therefore `implemented`, not
`assured`. W-000017 remains the PASS observation of the historical, narrower
O-000015 rule.

## Residual

No assurance is claimed until O-000024 has a valid witness for every declared
Scope clause. W-000026 makes the current generic non-colliding numeric,
feature-gated RFC3339-acceptance, and shared-adapter-isolation evidence
absence visible; that boundary does not narrow this Promise. No live-network
flow has been observed. Epoch-collision inputs and values are rejected under
P-000016 rather than assured here, and this Promise does not cover provider
meaning beyond the registration field or `client_id_issued_at`.

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
