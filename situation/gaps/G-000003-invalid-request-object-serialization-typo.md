# G-000003 — invalid_request_object serialization typo (upstream defect)

## State

closed

## Gap

Upstream defect, present verbatim at pin
b639b5d39eac6903238867aeb2b29326502e6b26 (tag 4.0.1) and carried
faithfully into this import: the serialization of
`CoreAuthErrorResponseType::InvalidRequestObject` emits
`invalid_request_obbject` (double `b`), while the `FromStr` arm accepts
the spec string `invalid_request_object`. The variant therefore does not
round-trip, and the outbound error code for this variant is wrong.

## Relevance

Surfaced by the CodeRabbit round-1 review of PR #1 (thread
PRRT_kwDOUlFWIM6kr1f0) against the fidelity-audited openidconnect 4.0.1
import governed by
situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md.
That decision's zero-behavior-delta policy keeps carried upstream files
verbatim, so the defect is inherited by design rather than introduced
here; fixing it inside the import PR would itself violate the policy.

## Evidence

Observed in this repository on 2026-09-22 at bank1-integration head
7885cbcd28790c0e9737e0676bd8d6842951151d: src/core/mod.rs line 795 maps
`CoreAuthErrorResponseType::InvalidRequestObject` to
`"invalid_request_obbject"` inside the `AsRef<str>` impl that the
`serialize_as_str!` invocation on src/core/mod.rs line 763 uses for serde
output (the macro emits `serializer.serialize_str(self.as_ref())`,
src/macros.rs line 738), while src/core/mod.rs line 772 maps the
spec-compliant string `"invalid_request_object"` to
`CoreAuthErrorResponseType::InvalidRequestObject`. The same bytes verified
at the upstream pin: lines 772 and 795 of
https://raw.githubusercontent.com/ramosbugs/openidconnect-rs/b639b5d39eac6903238867aeb2b29326502e6b26/src/core/mod.rs.
Interpretation, distinct from the observations above: because
`invalid_request_obbject` is not a `from_str` arm, the serialized string
read back falls into the extension arm
(src/core/mod.rs line 783, `CoreAuthErrorResponseType::Extension`),
so a round-trip yields a different variant rather than an error.

## Impact

A relying party that echoes the parsed error code outbound emits
`invalid_request_obbject`, a string no specification defines, to its
authorization server. Incoming spec-compliant `invalid_request_object`
strings still map correctly, so inbound handling is unaffected. The
standing gate suite passed with the typo present (see
situation/witnesses/P-000001/W-000001-import-head-offline-parity-leg-p6.md),
so no committed test pins the spec-correct serialization of this variant.

## Resolution

closed — corrected deliberately under
situation/decisions/D-000010-correct-invalid-request-object-serialization.md:
the outbound mapping now emits the spec string `invalid_request_object`, and a
consumer-observable round-trip regression test
(`src/core/tests.rs::test_auth_error_type_round_trip`) pins both directions —
serialization output and parsing of that output (and of the spec string) back
to the variant. The change is a named divergence from the import pin; future
upstream syncs must re-apply or consciously re-adjudicate it (D-000010).

## References

- situation/references/D-000004/R-000001-upstream-pin.md — the upstream pin
  carrying the defect verbatim.
- situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md —
  the fidelity policy that kept the fix out of the import PR.
- situation/decisions/D-000010-correct-invalid-request-object-serialization.md —
  the predeclared decision governing the correction.

## Provenance

Recorded 2026-09-22 from CodeRabbit round-1 review thread
PRRT_kwDOUlFWIM6kr1f0 on PR #1; the line-level defect verified the same
day in this repository at bank1-integration head
7885cbcd28790c0e9737e0676bd8d6842951151d and against upstream pin
b639b5d39eac6903238867aeb2b29326502e6b26 (tag 4.0.1).
