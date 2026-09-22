# D-000010 — Correct the InvalidRequestObject serialization typo

## Status

accepted

## Date

2026-09-22

## Context

G-000003 records an upstream defect carried verbatim at the import pin and
therefore inside the fidelity policy of
situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md:
the serialization of `CoreAuthErrorResponseType::InvalidRequestObject` emits
`invalid_request_obbject` (double `b`), a string no specification defines,
while the `FromStr` arm accepts the spec string `invalid_request_object`. The
variant does not round-trip: its serialized form parses back as
`CoreAuthErrorResponseType::Extension`. PR #2 of this repository schedules
this correction, and this Decision is recorded before the code change that
executes it.

## Evidence

- situation/gaps/G-000003-invalid-request-object-serialization-typo.md — the
  line-level observations: outbound mapping at `src/core/mod.rs` line 795,
  inbound arm at line 772, extension fallthrough at line 783, and the same
  bytes verified at upstream pin
  b639b5d39eac6903238867aeb2b29326502e6b26 (tag 4.0.1).
- The standing gate suite passed with the typo present
  (situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md),
  so no committed test pins the spec-correct serialization; the defect is
  observable only through the public serialization surface.
- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md
  — the zero-behavior-delta policy that kept the fix out of the import PR and
  classifies every future pin diff; this correction is a deliberate,
  decision-backed delta of a kind that policy does not enumerate.

## Decision

Correct `CoreAuthErrorResponseType::InvalidRequestObject` to serialize as the
spec string `invalid_request_object` (the same string its `FromStr` arm
already accepts), and add a consumer-observable round-trip regression test
that pins both directions: serialization output and `FromStr`/serde parsing
of that output back to the variant. This change lands as its own commit,
separate from documentation, test-only, and record commits.

This Decision reconciles the fidelity boundary of D-000004 for exactly this
one variant: the change is a deliberate behavior correction inherited-defect
fix, carried forward as a named divergence from the pin. D-000004's
classification table is not reopened; future upstream syncs must re-apply
this correction or consciously re-adjudicate it through a new Decision, not
silently revert to the upstream bytes.

## Why

A relying party echoing the parsed error code outbound currently emits
`invalid_request_obbject` to its authorization server — an interoperability
defect in a public API of a maintained continuation crate. Inbound handling
is unaffected, so the correction's blast radius is the outbound string only.
The continuation's purpose (a maintained, spec-conforming crate) outweighs
byte fidelity for this variant; keeping a known spec violation to preserve
diff-mechanics would subordinate consumer behavior to re-sync convenience.

## Rejected alternatives

- Keep the typo for byte fidelity: rejected because it knowingly emits a
  spec-undefined error code (G-000003 Impact) and breaks the variant's
  round-trip.
- Fix the string without a Decision or regression test: rejected because a
  behavior change without predeclared decision-backed records is exactly the
  drift the import fidelity policy exists to prevent.
- Rename the variant or change inbound parsing: rejected as API breakage far
  beyond the defect; inbound mapping is already spec-correct.

## Consequences

The public serialization of `CoreAuthErrorResponseType::InvalidRequestObject`
changes from `invalid_request_obbject` to `invalid_request_object`.
Round-tripping the variant through serialize/parse yields the same variant
again. The committed suite grows one round-trip regression test. Diffing
`src/core/mod.rs` against the pin now shows this deliberate divergence,
recorded here and in G-000003's resolution.

## Revisit when

Upstream changes or fixes this serialization (re-adjudicate against the new
pin), or the error-response-type representation is reworked structurally.
