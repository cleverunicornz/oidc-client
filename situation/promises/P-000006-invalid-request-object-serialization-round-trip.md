# P-000006 — InvalidRequestObject serialization round-trips

## State

assured

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

State `assured` cites
situation/oracles/O-000007-judge-invalid-request-object-serialization-round-trip.md
and the complete PASS witness
situation/witnesses/P-000006/W-000005-invalid-request-object-serialization-round-trip.md,
observed 2026-09-22 at head
`6bc21cdbcf933bcaf52a9f7f7ef33e3dda851bb3`: its scoped declared Cargo test
passed every Oracle Pass leg. The implementation originates in commit
`f37c32447b099e72a0b6bfeccf32556b02c9bd6c`; D-000010 records the selected
behavior correction and G-000003 retains the pre-correction observation.

## Residual

This promise does not assure the conversions of other
`CoreAuthErrorResponseType` variants or behavior outside the enum's
serialization and parsing surface.

## References

- situation/decisions/D-000010-correct-invalid-request-object-serialization.md
- situation/gaps/G-000003-invalid-request-object-serialization-typo.md
- src/core/tests.rs::test_auth_error_type_round_trip
- situation/witnesses/P-000006/W-000005-invalid-request-object-serialization-round-trip.md
