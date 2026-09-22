# P-000006 — InvalidRequestObject serialization round-trips

## State

implemented

## Promise

A consumer serializing `CoreAuthErrorResponseType::InvalidRequestObject` receives
the protocol string `invalid_request_object`; both that serialized value and the
protocol string parse back to the same enum variant.

## Scope

The public serde JSON serialization and serde/serde-plain parsing conversions
for `CoreAuthErrorResponseType::InvalidRequestObject`. This promise does not
cover any other authorization-error variant, an authorization-server deployment,
or a future upstream synchronization.

## Oracle

situation/oracles/O-000007-judge-invalid-request-object-serialization-round-trip.md

## State evidence

State `implemented` is supported by commit
`f37c32447b099e72a0b6bfeccf32556b02c9bd6c`, which changes the outbound mapping
to `invalid_request_object` and adds
`src/core/tests.rs::test_auth_error_type_round_trip`. D-000010 records the
selected behavior correction for the inherited defect; G-000003 retains the
pre-correction observation.

## Residual

This promise does not assure the conversions of other
`CoreAuthErrorResponseType` variants or behavior outside the enum's
serialization and parsing surface.

## References

- situation/decisions/D-000010-correct-invalid-request-object-serialization.md
- situation/gaps/G-000003-invalid-request-object-serialization-typo.md
- src/core/tests.rs::test_auth_error_type_round_trip
