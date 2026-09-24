# D-000014 — EC key-type doc correction and scoped import-fidelity supersession for the JWK surface

## Status

accepted

## Date

2026-09-23

## Context

Pre-publication hardening of the continued crate found two defects on the
`src/core/jwk` surface, both inherited from the openidconnect 4.0.1 pin
(`b639b5d39eac6903238867aeb2b29326502e6b26`):

1. `CoreJsonWebKey`'s derived `Debug` printed the full bytes of the EC/RSA
   private member (`d`) and the symmetric secret (`k`). The bytes render as a
   decimal list because `new_type!` (`src/macros.rs`) derives `Debug` on
   `Base64UrlEncodedBytes`, so log lines, panic messages, and test failures
   disclosed key material.
2. The `CoreJsonWebKeyType::EllipticCurve` variant documentation claimed
   "ECC algorithms such as ECDSA are currently unsupported", contradicting
   the assured verification paths: P-000002 (ES256, witness W-000003) and
   P-000007 (HMAC SHA-2 and ES384, witness W-000006), dispatched in
   `src/core/jwk/mod.rs` (`EcdsaP256Sha256`/`EcdsaP384Sha384` arms) to
   `crypto::verify_ec_signature`.

Under D-000004 every byte-delta from the pin must be classified
(rename / metadata / mechanical-lint / allow-addition). These changes are
deliberate maintenance deltas of the continued crate — a behavior change
(redaction) and a docs correction — categories D-000004 deliberately does not
carry, because it governs import fidelity at the pin. D-000004 is append-only
and must not be edited, so this decision provides the classification home.

## Evidence

- `src/core/jwk/mod.rs` — `CoreJsonWebKey` fields `d`/`k`; the
  `EcdsaP256Sha256`/`EcdsaP384Sha384` dispatch arms;
  `CoreJsonWebKeyType::EllipticCurve` doc.
- `src/macros.rs` — `new_type!` derives `Debug` (decimal byte-list output) on
  `Base64UrlEncodedBytes`.
- `grep` over `src/` and `tests/`: no user-visible error string formats a
  `CoreJsonWebKey` or `JsonWebKeySet` with `{:?}`; the exposure is the
  derived `Debug` itself.
- situation/gaps/G-000007-es256-public-docs-contradict-implementation.md —
  closed; its resolution corrected the per-algorithm enum docs in
  `src/core/mod.rs`. The key-type variant doc is a distinct missed location,
  not a reopening.
- situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md and
  situation/promises/P-000007-hmac-sha2-and-es384-jws-verification.md — the
  assured EC verification contracts the corrected doc must match.
- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the import-fidelity regime this decision scopes for one surface.
- Focused validation was reported at stream-worktree head `cdfe1ea`:
  `cargo test --offline --lib core::jwk` → 26 passed, 0 failed;
  `cargo check --offline` clean; a falsification run with the pre-fix derived
  `Debug` restored failed exactly the four new redaction tests. That head is
  unresolvable on this branch; G-000029 retains the provenance concern, while
  the landed implementation is commit `5ab6d7b`.

## Decision

1. Replace the derived `Debug` on `CoreJsonWebKey` with a manual
   implementation rendering `d` and `k` presence-only (`Some([redacted])` /
   `None`) while printing all non-secret fields normally. All other derives
   (`Clone`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`) and behavior are
   unchanged; serialization remains the export path for `d`/`k`.
2. Correct the `EllipticCurve` documentation: EC keys with the P-256 and
   P-384 curves are supported for JWS signature verification with the
   corresponding ECDSA algorithms (`ES256`, `ES384`); other curves such as
   P-521 (`ES512`) are currently unsupported. No signed-UserInfo, JWE, or
   signing claim is introduced. `CoreJsonCurveType::P521`'s accurate
   "(currently not supported)" label and the corrected per-algorithm docs in
   `src/core/mod.rs` are untouched.
3. Classification of this stream's deltas — manual `Debug` impl +
   `RedactedKeyMaterial` helper, the doc correction, and the five focused
   tests in `src/core/jwk/tests.rs`: deliberate maintenance deltas of the
   continued crate. For the touched surface (`src/core/jwk/mod.rs`,
   `src/core/jwk/tests.rs`) this decision supersedes the import-fidelity
   regime of D-000004: a future diff against pin `b639b5d3` must treat this
   surface as continued-crate maintenance rather than unclassified upstream
   drift. D-000004 itself remains accepted and unmodified for every other
   surface, and its append-only law is respected.
4. The new behavior is carried by
   situation/promises/P-000013-jwk-debug-redacts-secret-key-material.md and
   situation/oracles/O-000014-judge-jwk-debug-redacts-secret-key-material.md.
   W-000016 subsequently records a PASS on the named fixed-fixture legs at
   `b96b920f52e0d8b392edcf0b5d752fa570c2b356`; P-000013 remains
   `implemented` because that oracle does not yet decide the Promise's
   universal no-disclosure wording.

## Why

Secret bytes on `Debug` — the library's most incidental output path — are a
disclosure hazard with no observability benefit, and public rustdoc that
falsifies assured behavior (P-000002, P-000007) misleads consumers exactly
where G-000007 showed consumers read these claims. Redaction must preserve
presence (diagnosing a missing `d`/`k` stays possible) and must not touch
`PartialEq` or serde (key matching and export semantics). A scoped
supersession is the only mechanism that classifies deliberate continuation
maintenance without rewriting append-only history.

## Rejected alternatives

- Redacting the entire `CoreJsonWebKey` `Debug` output: destroys legitimate
  observability of non-secret fields (kid, kty, crv, public members).
- A quoted `"[redacted]"` string as the marker: rejected in favor of an
  unquoted `[redacted]` token so the marker cannot be mistaken for key data.
- Redacting public members (`n`, `e`, `x`, `y`): they are public key
  material; hiding them breaks JWK diagnosability with no secrecy benefit.
- Editing D-000004 to add a maintenance-delta category: violates the
  append-only law for decisions.
- Reopening G-000007: it is closed and its scope (per-algorithm enum docs in
  `src/core/mod.rs`) was correctly fixed; this key-type doc is a distinct
  location the closure did not cover.
- Altering `CoreJsonCurveType::P521`'s label or the `deny.toml` RSA
  disposition: out of scope; both are accurate as they stand.

## Consequences

`Debug` output of `CoreJsonWebKey` is now identical for keys differing only in
`d`/`k` bytes while `PartialEq` still distinguishes them; consumers relying on
`Debug` to read secret bytes (none found in-repo) would break by design.
A future upstream re-sync classifies this surface through this decision.
P-000013 remains `implemented`: W-000016 passes O-000014's named
fixed-fixture legs, but that oracle does not yet decide the Promise's
universal no-disclosure wording.

## Revisit when

The next upstream sync (re-derive the classification of this surface), if a
consumer needs a machine-readable redaction marker, or if P-000002/P-000007's
assurance scope changes so the corrected doc's support statement (P-521/ES512
unsupported) no longer holds.

## Provenance

Recorded 2026-09-23 by Stream C of the pre-publication hardening batch. The
stream reported worktree head `cdfe1ea`, but the reachable branch commit
carrying the implementation is `5ab6d7b`; see
situation/gaps/G-000029-stream-head-shas-unresolvable-on-branch.md for the
unresolved provenance boundary.
