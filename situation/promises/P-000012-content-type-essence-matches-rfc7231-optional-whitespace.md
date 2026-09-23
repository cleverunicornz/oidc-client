# P-000012 — Content-Type essence matching accepts RFC 7231 optional whitespace

## State

implemented

## Promise

Content-Type essence matching accepts legal RFC 7231 media types: the essence
(`<type>/<subtype>`) matches case-insensitively and tolerates optional whitespace (SP/HTAB)
around the type/subtype and before parameters, so `application/json ; charset=utf-8`,
`Application/JSON`, and `\tapplication/json\t` all match `application/json`. Whitespace
inside the type/subtype (for example `appl ication/json`), different or lookalike media
types (for example `application/jsonx`, `text/plain`, `application/jwt` when JSON is
expected), and non-UTF-8 header values remain rejected. The missing-Content-Type tolerance
in `check_content_type` and the JSON-default behavior for user info responses are unchanged.

## Scope

`content_type_has_essence` in `src/http_utils.rs` and every consumer of it:
`check_content_type` (used by discovery/JWKS fetching and registration responses) and the
user info JSON-vs-JWT response routing in `src/user_info.rs`. Outside Scope: parameter value
parsing (only the essence is compared) and header transport fidelity below the
`HeaderValue` layer.

## Oracle

situation/oracles/O-000013-judge-content-type-essence-matching.md

## State evidence

State `implemented` cites the Stream B pre-publication-hardening commit on branch
`fix/prepublication-hardening` that carries this record together with the implementation and
the focused tests named by the oracle
(situation/decisions/D-000015-prepublication-hardening-maintenance-deltas-http-header-surface.md
authorizes the deltas). Witnesses have not been collected yet; the parent's final gate
attaches them under `situation/witnesses/P-000012/` and applies the oracle to decide the
disposition.

## Residual

Only ASCII case variance and SP/HTAB whitespace are exercised; other legal-but-unusual field
value paddings are not. The allocation-free implementation (no lowercased copies) is an
implementation property, not a separately judged behavior.

## References

- src/http_utils.rs (`content_type_has_essence`, `check_content_type`; tests module)
- src/user_info.rs (`user_info_response` routing;
  `tests::test_user_info_response_routes_on_content_type_with_optional_whitespace`)
- src/registration/tests.rs (`test_registration_sends_bearer_header_and_accepts_ows_content_type`)
- https://tools.ietf.org/html/rfc7231#section-3.1.1.1 (adopted grammar reference)
