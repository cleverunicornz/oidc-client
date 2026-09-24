# D-000018 — Reject client-secret expiry values that collide with the never-expires sentinel

## Status

accepted

## Date

2026-09-24

## Context

`ClientSecretExpiration` reserves the numeric JSON value `0` for
`NeverExpires`. After the initial three-state representation was introduced,
the review correction in commit
`f80568881fccd9e32ad99f593b39b6e54f9fa59a` found a remaining collision:
an `ExpiresAt` value at the Unix epoch (including a subsecond value that the
shared timestamp adapter rounds down) could serialize as numeric `0`, then
deserialize as `NeverExpires`. Likewise, fractional numeric and feature-gated
RFC 3339 inputs that resolve to the epoch second could enter a representation
that cannot round-trip without changing state.

D-000017 remains the accepted decision for the primary absent / never-expires /
expires-at representation. This decision records the narrower collision rule
that makes its three states unambiguous.

## Evidence

- `src/registration/mod.rs` — `ClientSecretExpiration` owns the type-scoped
  serializer and deserializer; the shared `Timestamp` adapter remains outside
  the sentinel rule.
- `src/registration/tests.rs` — the epoch, subsecond, fractional numeric, and
  feature-gated RFC 3339 collision tests added by
  `f80568881fccd9e32ad99f593b39b6e54f9fa59a`.
- situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md
  and situation/oracles/O-000015-judge-client-secret-expiration-semantics.md
  — the already witnessed three-state core, whose stated scope must not be
  broadened by an unobserved collision correction.

## Decision

1. Numeric `0` remains the sole serialized form of
   `ClientSecretExpiration::NeverExpires`. An `ExpiresAt` value that resolves
   to the Unix epoch second is rejected during serialization instead of being
   emitted as `0`.
2. A numeric or feature-gated RFC 3339 input that resolves to the epoch second
   is rejected during deserialization instead of becoming either
   `ExpiresAt` or `NeverExpires`.
3. The rule remains local to `ClientSecretExpiration`; neither the shared
   `helpers::Timestamp` adapter nor its other consumers change.
4. The correction is carried by
   situation/promises/P-000016-client-secret-expiration-rejects-never-expires-sentinel-collisions.md
   and situation/oracles/O-000017-judge-client-secret-expiration-sentinel-collisions.md.

## Why

A public enum that silently serializes one variant into the wire sentinel for a
different variant destroys the state distinction that the registration API was
created to expose. Rejection is the only behavior that preserves round-trip
truth without changing the OpenID Connect wire sentinel or extending its
meaning to unrelated timestamps.

## Rejected alternatives

- Silently serialize colliding `ExpiresAt` values as `0`: rejected because a
  consumer receives `NeverExpires` after a round trip and loses the original
  expiry state.
- Normalize a colliding input to `NeverExpires`: rejected because an expiry
  value and a non-expiring secret are observably different semantic states.
- Special-case numeric `0` in `helpers::Timestamp`: rejected because that
  adapter serves unrelated claims for which the epoch is not a never-expires
  sentinel.

## Consequences

Consumers constructing an epoch-colliding `ExpiresAt` receive a serde error and
must choose `NeverExpires` when that is their intended state. Providers sending
an epoch-colliding non-sentinel form receive a deserialization error. The
existing P-000014 assurance remains limited to the representable three-state
core; P-000016 owns this correction's assurance boundary.

## Revisit when

The OpenID Connect Dynamic Client Registration specification changes the
numeric `0` sentinel, or `ClientSecretExpiration` gains a representation that
can preserve epoch expiry without aliasing it to `NeverExpires`.
