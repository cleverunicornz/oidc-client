# D-000012 — No crate-provided discovery cache in the 4.1 lineage

## Status

accepted

## Date

2026-09-22

## Context

P-000001's earlier wording claimed discovery "with caching", but neither the
imported 4.0.1 donor nor this repository's tree contains discovery-cache
behavior. G-000005 records the absence and asks whether the crate should
provide one or the promise wording must stay narrow.

## Evidence

- Exact case-insensitive `git grep -il cache` over `src/` and `tests/` at the
  donor boundary `92946acf13d02a67caab38cb64444a902217fae4` (the BACKPORT
  opening checkpoint whose tree is historical donor material) returns no
  file; the same search over the current tree returns no source file.
- `src/discovery/mod.rs` defines provider-metadata discovery and parsing;
  `src/discovery/tests.rs` covers deserialization only.
- `src/discovery/mod.rs` takes the HTTP layer as a generic
  `SyncHttpClient`/`AsyncHttpClient` parameter, so a consumer controls every
  discovery transport call and can cache at that boundary.
- situation/decisions/D-000002-version-oidc-client-4-1-0-continuing-upstream-lineage.md —
  version 4.1.0 is one minor step above upstream 4.0.1 and signals drop-in
  compatibility with the donor baseline.
- situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md —
  the corrected Promise and Scope no longer claim caching and place a
  crate-provided discovery cache outside scope.

## Decision

No crate-provided discovery cache is added to the 4.1 lineage. Discovery
remains fetch-parse-construct per invocation, matching the imported upstream
baseline; caching a provider's discovered metadata is the consumer's
responsibility, exercised at the pluggable HTTP-client boundary. Adding cache
behavior would be a new feature outside the 4.1.x continuity contract and
requires its own Candidate, Promise, and Oracle lineage rather than a gap
remedy.

## Why

The 4.1.0 version contract is the upstream 4.0.1 behavior plus this
repository's recorded additions (D-000002). The upstream baseline has no
discovery cache, so a cache cannot be part of the drop-in compatibility story
the version signals; shipping one would extend the public surface beyond the
assured baseline under a version number that promises continuity. The
persistence policy is also provider- and deployment-specific, which is
consumer territory, not crate territory.

## Rejected alternatives

- Implement a discovery cache in this crate now: rejected — new behavior the
  donor never had, outside 4.1.x continuity, requiring its own assurance
  lineage; it cannot be discharged as a gap remedy.
- Retain "with caching" in P-000001's wording: rejected — it claims behavior
  that no source occurrence, test, or witness supports (G-000005's evidence).

## Consequences

G-000005 becomes `accepted` as a documented limitation: this crate does not
persist or reuse discovery documents, and consumers own caching. P-000001's
scope wording stays as corrected. The absence is part of the published
contract, not a defect against it.

## Revisit when

A consumer requirement or an upstream 4.x baseline feature makes cache
behavior part of the continuity story, in which case a new Candidate
qualifies the feature with its own Promise and Oracle.

## Provenance

Recorded 2026-09-22 by the pre-publication bookkeeping lane (PR #3, branch
`bank3/bookkeeping`) to settle G-000005; the searches were re-derived from
the donor boundary and the current tree the same day.
