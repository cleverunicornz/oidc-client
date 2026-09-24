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

State `implemented` cites implementation commit
`60c11639afc4d0b7504d7338c1a810f8c38eee30`, which changes the shared
essence comparison and adds the focused consumer-path tests named by
O-000013. The Promise, Oracle, and Witness records were attached
retrospectively in `9c4013a9d93a93e65084b3475022c43059e64fc2`.

situation/witnesses/P-000012/W-000015-content-type-essence-optional-whitespace.md
is a PASS observation from the parent's final gate at
`b96b920f52e0d8b392edcf0b5d752fa570c2b356` and decides each named
O-000013 leg. The state remains `implemented`, not `assured`: the named
fixtures cover the listed case and OWS forms, not every legal-but-unusual
field-value padding retained in Residual.

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
