# D-000017 — Pre-publication hardening maintenance delta for the registration client-secret expiry surface

## Status

accepted

## Date

2026-09-23

## Context

The pre-publication review of the 4.1.0 continuation line identified a
defect on the dynamic-registration response surface:
`ClientRegistrationResponse::client_secret_expires_at` typed the wire
value through the shared `helpers::Timestamp` adapter as
`Option<DateTime<Utc>>`, so the OpenID Connect Dynamic Client Registration
2 sentinel `0` ("the client secret DOES NOT expire") deserialized to
`Some(1970-01-01T00:00:00Z)` — an already-expired epoch date — and the
setter could construct the same ambiguity. Interpreting the field
required knowing an undocumented sentinel, and no type distinguished
absent / never / expires-at. Fixing it changes donor-derived bytes and a
public API shape, which D-000004's import-fidelity regime classifies as
defects unless authorized; the crate is unpublished, so a complete
public-API break is acceptable pre-publication.

## Evidence

- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the import-fidelity regime this decision scopes itself against.
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
  and situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
  — the established maintenance-delta pattern this decision follows.
- `src/helpers.rs` — the shared `Timestamp` adapter (`Seconds` variant,
  feature-gated `Rfc3339` variant, floor-rounding in `to_utc`) reused by
  `client_id_issued_at`, ID-token `exp`/`iat`/`auth_time`, and the
  verification tests; the never-expires sentinel is specific to
  `client_secret_expires_at`.
- `grep` over the repository: `client_secret_expires_at` appears only in
  `src/registration/mod.rs`, `src/registration/tests.rs`, and
  `tests/rp_certification_dynamic.rs`; no other callers exist.
- The Stream D focused run `cargo test --offline --lib registration`:
  11 passed, 0 failed (4 new tests, 7 pre-existing including the wave-2
  bearer/Content-Type lanes); `cargo check --offline --tests` clean (the
  `#[ignore]`d live certification test compiles unchanged); the four new
  tests also pass under `--features accept-rfc3339-timestamps`; a
  falsification run with the zero-sentinel check disabled failed exactly
  `test_client_secret_expiration_never_expires`.
- The pre-publication review findings directed by the parent session
  (branch `fix/prepublication-hardening`).

## Decision

1. The following deltas are deliberate maintenance deltas of the
   continued crate, classified as `maintenance-feature` (a new public
   representation of existing wire behavior on an owned continuation
   line), superseding D-000004's regime for exactly this surface and no
   other: `src/registration/mod.rs` (the public `ClientSecretExpiration`
   enum — `NeverExpires` / `ExpiresAt(DateTime<Utc>)` — with manual
   `Serialize`/`Deserialize` impls scoped to this type: numeric `0` →
   `NeverExpires`; a supported timestamp that resolves to a Unix epoch second
   other than `0` → `ExpiresAt` through the shared `Timestamp` adapter's
   floor-rounding; `NeverExpires` → numeric `0`; and a colliding epoch form is
   rejected under D-000018; the response field type becomes
   `Option<ClientSecretExpiration>`; getter and setter migrated),
   `src/registration/tests.rs` (migrated real-timestamp assertion; four
   new focused tests), and no change to `tests/rp_certification_dynamic.rs`
   (its `log_field!` calls the getter and formats with `Debug`, which the
   new type derives — compile-verified). D-000004 itself remains
   unchanged and authoritative for every other surface.
2. The representation is a two-variant enum inside `Option`, keeping
   three distinct states — absent (`None`), never expires
   (`NeverExpires`), expires at (`ExpiresAt`). The serde customization
   lives only on the new type; the shared `helpers::Timestamp` adapter
   and all its other consumers are untouched.
3. Pre-publication API break, accepted: the field type changes from
   `Option<DateTime<Utc>>` to `Option<ClientSecretExpiration>` and the
   getter returns `Option<&ClientSecretExpiration>` instead of
   `Option<DateTime<Utc>>`. The crate is unpublished (not on crates.io);
   every in-repository caller is migrated in the same unit. UPGRADE.md
   stays at verbatim upstream bytes; this decision and the parent's PR
   description record the break instead. `src/lib.rs` is unchanged:
   registration types are exported through `pub mod registration` and no
   crate-root registration re-export list exists to extend.

## Why

The wire contract already distinguishes three states, so the type system
should carry all three; and the sentinel belongs to
`client_secret_expires_at` alone, so handling it inside the shared
timestamp adapter would change the meaning of numeric `0` for every other
claim, where `0` is either invalid or an ordinary instant. Reusing the
shared adapter inside the new type's deserializer keeps real-timestamp
behavior (floor-rounding, RFC 3339 feature acceptance) identical to
before.

## Rejected alternatives

- Keeping `Option<DateTime<Utc>>` and documenting the `0` sentinel for
  consumers: leaves the undocumented-sentinel hazard the review flagged;
  every consumer must reimplement the interpretation and can misread the
  field as already expired.
- A newtype around `DateTime<Utc>` or around `Option<DateTime<Utc>>`:
  still cannot distinguish never-expiring from the epoch instant without
  the same sentinel check at every use site.
- A string sentinel (e.g. serializing `"never"`): departs from the
  numeric wire format and breaks round-trip fidelity with providers that
  send `0`.
- Handling `0` inside `helpers::Timestamp` (mapping it to a special
  `DateTime` or an `Option`): changes every `Timestamp` consumer
  (`client_id_issued_at`, ID-token `exp`/`iat`/`auth_time`), where the
  numeric `0` has no never-expires meaning.
- Adding a crate-root re-export for the new type in `src/lib.rs`: no
  registration re-export list exists at the crate root;
  `oidc_client::registration::ClientSecretExpiration` matches how the
  other registration types are consumed.

## Consequences

Future diffs against the donor pin over this surface are judged against
this decision (maintenance deltas) rather than flagged as fidelity
defects. Consumers observe the API break (field type, getter and setter
signatures) and now match on `ClientSecretExpiration` variants; numeric
`0` responses no longer appear as expired 1970 timestamps, and
serialization round-trips with numeric-`0` senders. New public API:
`ClientSecretExpiration` (variants `NeverExpires`, `ExpiresAt`).
situation/promises/P-000014-client-secret-expiration-never-expires-semantics.md
and situation/oracles/O-000015-judge-client-secret-expiration-semantics.md
carry the witnessed three-state core; P-000014 is `assured` by
situation/witnesses/P-000014/W-000017-client-secret-expiration-semantics.md
at `b96b920f52e0d8b392edcf0b5d752fa570c2b356`. The complementary
epoch-collision rule is selected by D-000018 and carried by P-000016/O-000017.

## Revisit when

An upstream re-sync touches the registration response surface (re-derive
the delta table), or the OIDC Registration specification changes the `0`
sentinel or the field's semantics.

## Provenance

Recorded 2026-09-23 by Stream D (RegistrationExpiry) of the
pre-publication hardening batch: implementation landed in
`f93c8c79055fa5caf5adb6690d9dec123974b702`, and the Promise, Oracle, and
Witness records were attached retrospectively in
`9c4013a9d93a93e65084b3475022c43059e64fc2`. Corrected 2026-09-24 in place
under the open-PR correction rule to delimit the later
`f80568881fccd9e32ad99f593b39b6e54f9fa59a` epoch-collision change; D-000018
records that selected boundary.
