# O-000007 — Judge InvalidRequestObject serialization round-trip

## State

implemented

## Judges

situation/promises/P-000006-invalid-request-object-serialization-round-trip.md

## Inputs

`src/core/tests.rs::test_auth_error_type_round_trip` and its scoped
`cargo test --all-features test_auth_error_type_round_trip` execution.

## Pass

- P1: Serde JSON serialization of
  `CoreAuthErrorResponseType::InvalidRequestObject` yields exactly
  `"invalid_request_object"`.
- P2: Serde JSON parsing of that serialized value yields
  `CoreAuthErrorResponseType::InvalidRequestObject`.
- P3: Serde-plain parsing of `invalid_request_object` yields
  `CoreAuthErrorResponseType::InvalidRequestObject`.

## Fail

- F1: The JSON serialization output differs from
  `"invalid_request_object"`.
- F2: Parsing the JSON serialization output does not yield
  `CoreAuthErrorResponseType::InvalidRequestObject`.
- F3: Parsing `invalid_request_object` through serde-plain does not yield
  `CoreAuthErrorResponseType::InvalidRequestObject`.

## Implementation

`cargo test --all-features test_auth_error_type_round_trip` executes the
consumer-observable serialization and parsing assertions.

## Implementation coverage

| Leg | Decision | Coverage |
|---|---|---|
| P1 | The exact JSON string is compared with `"invalid_request_object"`. | `src/core/tests.rs::test_auth_error_type_round_trip` |
| P2 | The serde JSON result is compared with `InvalidRequestObject`. | `src/core/tests.rs::test_auth_error_type_round_trip` |
| P3 | The serde-plain parse result is compared with `InvalidRequestObject`. | `src/core/tests.rs::test_auth_error_type_round_trip` |
| F1 | A divergent JSON string fails the exact-string assertion. | `src/core/tests.rs::test_auth_error_type_round_trip` |
| F2 | A non-round-tripping JSON result fails the variant assertion. | `src/core/tests.rs::test_auth_error_type_round_trip` |
| F3 | A non-parsing serde-plain value fails the variant assertion. | `src/core/tests.rs::test_auth_error_type_round_trip` |
