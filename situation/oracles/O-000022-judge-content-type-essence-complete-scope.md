# O-000022 — Judge Content-Type essence complete scope

## State

implemented

## Judges

situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md

## Inputs

At the judged head, Content-Type header values passed to
`content_type_has_essence`, `check_content_type`, dynamic registration, and
UserInfo response routing. This successor retains the existing case and
representative OWS fixtures and adds a structural decision for every legal
SP/HTAB optional-whitespace sequence declared by the Promise.

## Pass

- P1 — Every legal sequence of SP and HTAB optional whitespace around the
  type/subtype or before the parameter delimiter is ignored when comparing an
  otherwise matching essence.
- P2 — The expected type/subtype matches case-insensitively.
- P3 — Internal-whitespace type/subtype strings and lookalike or wrong media
  types are rejected.
- P4 — A non-UTF-8 header value is rejected.
- P5 — `check_content_type` keeps missing-header tolerance, accepts an
  OWS-bearing expected value, and rejects a wrong media type.
- P6 — The public registration path accepts an OWS-bearing JSON response.
- P7 — The UserInfo router parses OWS-bearing JSON and routes OWS-bearing JWT
  Content-Type values to JWT verification rather than rejecting the header.

## Fail

- F1 — Any legal SP/HTAB optional-whitespace arrangement around a matching
  essence is rejected.
- F2 — An internal-whitespace, lookalike, wrong, or non-UTF-8 media type is
  accepted as the expected essence.
- F3 — `check_content_type`, registration, or UserInfo routing changes its
  declared response behavior because of an OWS-bearing Content-Type.
- F4 — Missing Content-Type tolerance or UserInfo's JSON default changes.

## Implementation

`cargo test --offline --lib --quiet -- content_type` executes the retained
finite fixtures through the helper and its consumers. The universal legal-OWS
leg is structurally manual because the grammar permits unbounded sequences.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | All legal SP/HTAB OWS around the essence is ignored. | manual — `src/http_utils.rs::content_type_has_essence` splits at `;` and applies `trim_matches` to every leading/trailing SP/HTAB byte of the essence slice |
| P2 | Case variance matches. | `src/http_utils.rs::test_content_type_has_essence_accepts_case_and_optional_whitespace` |
| P3 | Malformed and wrong essences reject. | `src/http_utils.rs::test_content_type_has_essence_rejects_unexpected_and_malformed_values` |
| P4 | Non-UTF-8 value rejects. | `src/http_utils.rs::test_content_type_has_essence_rejects_unexpected_and_malformed_values` |
| P5 | Helper consumer preserves missing/OWS/wrong behavior. | `src/http_utils.rs::test_check_content_type_tolerates_missing_header_and_accepts_ows` |
| P6 | Registration accepts OWS-bearing JSON. | `src/registration/tests.rs::test_registration_sends_bearer_header_and_accepts_ows_content_type` |
| P7 | UserInfo routes OWS-bearing JSON and JWT correctly. | `src/user_info.rs::test_user_info_response_routes_on_content_type_with_optional_whitespace` |
| F1 | Legal OWS never rejects. | manual — same structural `trim_matches` decision as P1 |
| F2 | Invalid media types never match. | P3 and P4 fixtures |
| F3 | Consumer routing remains correct. | P5–P7 fixtures |
| F4 | Missing/default behavior remains unchanged. | P5 and P7 fixtures |

## References

- Supersedes O-000013 for P-000012's complete declared scope; W-000015
  remains an observation of the historical, narrower rule.
- situation/witnesses/P-000012/W-000024-content-type-complete-scope-incomplete.md
