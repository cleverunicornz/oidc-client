# D-000006 — Recognize carried ES256 verification instead of reimplementing it

## Status

accepted

## Date

2026-09-22

## Context

Candidate C-000002 proposed adding ES256 verification to an imported
openidconnect 4.0.1 codebase. The admitted opening tree instead contains the
ES256 verification path already: it deserializes EC P-256 JWKs, dispatches
ES256 to `p256`, and unit-tests P-256 signature verification through that
dispatch. Treating that existing path as unimplemented would create duplicate
work and retain a false repository claim.

## Evidence

- `src/core/jwk/mod.rs` selects EC keys for
  `CoreJwsSigningAlgorithm::EcdsaP256Sha256` and routes a P-256 key to
  `crypto::verify_ec_signature`.
- `src/core/crypto.rs` constructs a `p256::ecdsa::VerifyingKey` from the EC
  JWK coordinates and verifies the JWS signature.
- `src/core/jwk/tests.rs` parses a P-256 EC JWK and tests valid and invalid
  P-256 signature verification through the ES256 enum dispatch.
- `situation/witnesses/evidence/W-000001/gates-final.log` records that
  `test_core_jwk_deserialization_ec` and `test_ecdsa_verification` passed at
  head `24835e4b44caa8a0baae2ac5b865bbd6bdf355ba`.

## Decision

Reject C-000002's proposal to add a new ES256 verification implementation.
Treat P-000002 as a retrospective record of the ES256 behavior already carried
from the donor, with assurance remaining subject to its own Oracle and a
complete Witness.

## Why

The opening-tree implementation and retained gate evidence settle the
implementation question. Reimplementing an existing path would add avoidable
risk to a security-sensitive verifier; calling a partial historical observation
assurance would overstate evidence.

## Rejected alternatives

- Reimplement ES256 around the existing path: rejected because the path already
  accepts a P-256 JWK, dispatches ES256 to `p256`, and rejects invalid
  P-256 signature inputs.
- Leave C-000002 proposed: rejected because it misstates a settled source fact
  as future work.
- Mark P-000002 assured from the existing gate log: rejected because no
  complete PASS witness applies O-000002 to every oracle leg.

## Consequences

C-000002 becomes rejected. P-000002 may accurately state that implementation
exists, but it remains unassured until O-000002 has a complete PASS Witness.
The retained upstream enum comments that describe ECDSA as unsupported are
historical donor prose and do not override the executed verification path.

## Revisit when

The ES256 verification behavior changes, or a new implementation is proposed
for an uncovered ES256 boundary.
