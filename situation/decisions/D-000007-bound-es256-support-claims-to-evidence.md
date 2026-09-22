# D-000007 — Bound ES256 support claims to observed evidence

## Status

accepted

## Date

2026-09-22

## Context

I-000002 previously called ES256 a first-class algorithm with equal test
coverage and documentation, and said discovery metadata includes it by
default. The admitted source does implement EC P-256 verification and parses
ES256 metadata, but its public enum comment still says ECDSA is unsupported,
and the retained ES256 witness is INVALID because no full ID-token fixture
covers the manual Oracle legs. The first-class assertion therefore exceeds the
recorded evidence.

## Evidence

- `src/core/jwk/mod.rs` and `src/core/crypto.rs` implement P-256 ES256
  verification.
- `src/core/mod.rs` labels ECDSA variants "currently unsupported" in the
  public enum documentation.
- `situation/witnesses/P-000002/W-000002-import-head-es256-unit-observation.md`
  records an INVALID partial observation under O-000002.
- `situation/oracles/O-000002-judge-es256-verification.md` retains manual
  P4/F4 full-ID-token legs.

## Decision

Supersede the unsupported first-class assertion in I-000002 with the binding
rule that repository-facing ES256 support claims remain bounded by P-000002's
implemented-but-unassured state and its named Oracle/Witness lineage.

## Why

The source supports a narrower factual implementation claim, while first-class
status would imply documentation and evidence that are not present. The record
system must preserve the useful implementation fact without treating it as
assured behavior.

## Rejected alternatives

- Leave I-000002 unchanged: rejected because it overstates test coverage,
  documentation, and assurance.
- Describe ES256 as unsupported: rejected because the EC JWK and P-256
  verification path exists and its unit tests pass.
- Mark P-000002 assured: rejected because W-000002 is INVALID and O-000002
  has manual full-ID-token legs without a PASS witness.

## Consequences

I-000002 becomes an evidence-bound support-claim invariant. The root repository
block contains no critical ES256 invariant, and README orientation may state the
implemented/unassured status rather than planned or first-class support.

## Revisit when

P-000002 is assured under O-000002 with a complete PASS Witness and the public
documentation has been brought into agreement with that evidence.
