# D-000015 — Pre-publication hardening maintenance deltas for the HTTP header surface

## Status

accepted

## Date

2026-09-23

## Context

The pre-publication review of the 4.1.0 continuation line identified two defects on the
HTTP header utility surface: `auth_bearer` ended in `.expect("invalid access token")`, so a
provider-issued access token containing a newline or other control byte deserialized fine
and then panicked at request preparation; and `content_type_has_essence` compared the raw
pre-`;` prefix lowercased, rejecting legal RFC 7231 optional whitespace
(`application/json ; charset=utf-8`) and allocating two lowercase Strings per check. Fixing
them changes donor-derived bytes, which D-000004's import-fidelity regime classifies as
defects unless authorized.

## Evidence

- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md —
  the import-fidelity regime this decision scopes itself against.
- situation/decisions/D-000013-prepublication-hardening-maintenance-deltas-user-info-surface.md
  and situation/decisions/D-000014-ec-key-type-doc-correction-and-scoped-import-fidelity-supersession.md
  — the established maintenance-delta pattern this decision follows.
- The http 1.5.0 crate: `HeaderValue::from_str` rejects control bytes;
  `InvalidHeaderValue`'s `Display` is the static string `failed to parse header value`;
  `impl From<InvalidHeaderValue> for http::Error` exists (http-1.5.0 src/error.rs:134).
- The oauth2 5.0.0 crate: `SyncHttpClient` / `AsyncHttpClient` blanket impls for closures
  (oauth2-5.0.0 src/endpoint.rs), enabling the recording mock clients used by the focused
  tests.
- The Stream B focused run `cargo test --offline --lib -- http_utils user_info registration
  verification`: 36 passed, 0 failed, including the wave-1 lanes
  `verification::tests::test_user_info_signed_response_es256` /
  `::test_user_info_signed_response_hs256` / `::test_user_info_subject_binding` /
  `::test_id_token_verification_key_at_hash` and
  `src/user_info.rs::test_user_info_request_signed_response_policy` (no regression).
- The pre-publication review findings directed by the parent session (branch
  `fix/prepublication-hardening`).

## Decision

1. The following byte deltas are deliberate maintenance deltas of the continued crate,
   classified as `maintenance-fix` (behavior repair on an owned continuation line),
   superseding D-000004's regime for exactly this surface and no other:
   `src/http_utils.rs` (fallible `auth_bearer`; RFC 7231 OWS-aware, allocation-free
   `content_type_has_essence`; new test module), `src/user_info.rs`
   (`UserInfoRequest::prepare_request` migration; new tests), `src/registration/mod.rs`
   (`prepare_registration` migration), and `src/registration/tests.rs` (new tests). D-000004
   itself remains unchanged and authoritative for all other imported bytes.
2. `auth_bearer` returns
   `Result<(HeaderName, HeaderValue), http::header::InvalidHeaderValue>`. The user info
   surface funnels the error through `From<InvalidHeaderValue> for http::Error` inside
   `prepare_request`, so it surfaces via the existing wrapping as
   `UserInfoError::Other("failed to prepare request: …")`. The registration surface
   transposes the optional header onto
   `ClientRegistrationError::Other("failed to prepare request: …")` inside
   `prepare_registration`, before any HTTP dispatch on both the synchronous and asynchronous
   paths. `src/http_utils` is a private module, so no public API changes.
3. No error path echoes credential bytes: `InvalidHeaderValue`'s `Display` is static text,
   token bytes are never formatted into any message, and the tests assert the negative.

## Why

A continuation line owns its donors' defects; the maintenance-delta classification keeps
the delta table auditable without freezing the API. Returning `InvalidHeaderValue` directly
is the smallest faithful type: the panic was never a type error, only an `expect` on a
normal input condition, and the existing per-surface error wrapping already carries context
(`failed to prepare request`) the consumers rely on. Trimming SP/HTAB and comparing with
`eq_ignore_ascii_case` implements exactly the RFC 7231 grammar the adjacent comment already
cited, without the per-check allocations.

## Rejected alternatives

- `catch_unwind` around header construction: masks the panic symptom, composes badly with
  consumers that themselves catch unwinds, and treats a normal input condition as a bug.
- Stripping or replacing invalid characters in the token: silently alters credentials and
  could transmit a different, attacker-meaningful token; authentication requires exactness.
- Returning `Option` and omitting the Authorization header on failure: sends an
  unauthenticated request that the server would judge on its own policy — strictly worse
  than failing.
- A custom error enum wrapping `InvalidHeaderValue`: new type surface for a private-module
  helper with no consumer need beyond Display; `InvalidHeaderValue` already implements
  `std::error::Error` and composes into both existing error paths.
- Keeping the lowercased-String comparison: two allocations per check on every response;
  `eq_ignore_ascii_case` is equivalent for the token grammar and allocation-free.

## Consequences

Future diffs against the donor pin over this surface are judged against this decision
(maintenance deltas) rather than flagged as fidelity defects. Consumers of the user info
and registration flows observe new `Err` returns (instead of panics) for malformed access
tokens, under the existing `UserInfoError::Other` / `ClientRegistrationError::Other`
variants; valid-token and tokenless behavior is unchanged. Responses with RFC 7231
whitespace/case variance in Content-Type are now accepted everywhere the essence is
checked. No public API changes: `src/http_utils` is crate-private.

## Revisit when

An upstream re-sync touches `src/http_utils.rs` (re-derive the delta table), or a consumer
need emerges to expose the header-construction error type (or the helper) publicly.

## Provenance

Recorded 2026-09-23 by Stream B (HttpHardening) of the pre-publication hardening batch.
