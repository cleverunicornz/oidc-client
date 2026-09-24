# O-000013 — Judge Content-Type essence matching

## State

implemented

## Judges

situation/promises/P-000012-content-type-essence-matches-rfc7231-optional-whitespace.md

## Inputs

The scoped Cargo test command below over `src/http_utils.rs`, `src/user_info.rs`, and
`src/registration/tests.rs`: unit fixtures of `HeaderValue` media types against
`content_type_has_essence` and `check_content_type`, a registration response with an
OWS-bearing Content-Type on the public `register` path, and user info mock responses routed
through `user_info_response`.

## Pass

- P1: `content_type_has_essence` accepts `application/json`, `Application/JSON`,
  `application/json ; charset=utf-8`, `application/json;charset=utf-8`, and
  `\tapplication/json\t` when expecting `application/json`.
- P2: It rejects `application/jsonx`, `application/jwt` (when expecting `application/json`),
  `appl ication/json`, and `text/plain`.
- P3: It rejects a non-UTF-8 `HeaderValue`.
- P4: `check_content_type` tolerates a missing Content-Type header, accepts an OWS-bearing
  value, and rejects a wrong media type; the public registration path accepts a response
  with `APPLICATION/JSON ; charset=UTF-8`.
- P5: The user info router parses JSON claims under `APPLICATION/JSON ; charset=utf-8` and
  routes `application/jwt\t` to JWT verification (an invalid JWT body fails parsing, not
  Content-Type matching).

## Fail

- F1: A legal RFC 7231 case/whitespace variant of the expected essence is rejected.
- F2: A lookalike or wrong media type, internal-whitespace type/subtype, or non-UTF-8 value
  is accepted.
- F3: The user info router treats an OWS-bearing Content-Type as unexpected, or routes it to
  the wrong branch.

## Implementation

`cargo test --offline --lib -- http_utils user_info registration verification` executes the
legs.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | Case and OWS variants accepted. | `src/http_utils.rs::test_content_type_has_essence_accepts_case_and_optional_whitespace` |
| P2 | Wrong and malformed essences rejected. | `src/http_utils.rs::test_content_type_has_essence_rejects_unexpected_and_malformed_values` |
| P3 | Non-UTF-8 values rejected. | `src/http_utils.rs::test_content_type_has_essence_rejects_unexpected_and_malformed_values` |
| P4 | check_content_type tolerance, acceptance, and public-path acceptance. | `src/http_utils.rs::test_check_content_type_tolerates_missing_header_and_accepts_ows`, `src/registration/tests.rs::test_registration_sends_bearer_header_and_accepts_ows_content_type` |
| P5 | OWS-bearing routing on the user info path. | `src/user_info.rs::test_user_info_response_routes_on_content_type_with_optional_whitespace` |
| F1 | Legal variants must not be rejected. | the P1 test (any rejection fails the test) |
| F2 | Lookalikes must not be accepted. | the P2 and P3 tests |
| F3 | Routing must not misroute. | the P5 test |

## References

- Superseded for P-000012's complete declared Scope by
  situation/oracles/O-000022-judge-content-type-essence-complete-scope.md.
  W-000015 remains an observation of this historical, narrower rule.
