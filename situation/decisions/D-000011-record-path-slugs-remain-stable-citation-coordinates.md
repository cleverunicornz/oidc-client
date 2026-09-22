# D-000011 — Record path slugs remain stable citation coordinates

## Status

accepted

## Date

2026-09-22

## Context

G-000012 observes that records corrected during the Bank 1 closure retain
path slugs describing superseded or rejected wording rather than the content
now stored at those paths:

- `situation/invariants/I-000002-es256-is-a-first-class-supported-algorithm.md`
  now bounds ES256 support claims by evidence and forbids representing ES256
  as first-class while unassured.
- `situation/plans/active/PLAN-000001-import-es256-publish-consume.md` now has
  the title and scope "Establish imported OIDC behavior and gate evidence"
  with no publication or consumption work.
- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
  now makes the narrower implemented-baseline claim.

Renaming the files would require an all-caller clean cutover.

## Evidence
- situation/gaps/G-000012-record-path-slugs-retain-rejected-claims.md — the
  observation and its impact analysis.
- Reference-count survey performed 2026-09-22 on this repository's tracked
  `*.md` files, first at PR-2 assurance-branch head f21b6d3 and refreshed at
  the then-current assurance-branch head 6c098f3 (source: `grep -rlF` over the
  working tree, excluding the cited files themselves and including this
  Decision's own three citations):
  `P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
  is cited by 12 files (decisions D-000003, D-000004, D-000011; gaps
  G-000005, G-000008, G-000012, G-000014; oracles O-000001, O-000004; plan
  PLAN-000001; witness W-000001; `README.md`),
  `I-000002-es256-is-a-first-class-supported-algorithm.md` by 2 (G-000012;
  D-000011), and `PLAN-000001-import-es256-publish-consume.md` by 2
  (G-000012; D-000011). The original survey counted 10 / 1 / 1 at f21b6d3;
  the lower bound of twelve cross-references remains valid — renaming now
  rewrites at least sixteen, several inside records already closed by the
  previous closure's checkpoint commits.
- situation/AGENTS.md ("Reference discipline") — repository-root-relative
  paths are the protocol's stable citation coordinates for records in this
  repository.

## Decision

Retain the existing record filenames unchanged and record this retention as
an explicit stability choice. Record content — title, State, and body —
governs meaning; the slug is a historical label of the record's creation.
I-000003 carries the resulting binding citation rule.

## Why

The paths are citation coordinates consumed by committed records and human
orientation; a rename buys no behavioral or factual correction while
rewriting closed records (and a README already reviewed at a closing
checkpoint), creating churn in the audit trail and merge surface mid-PR. The
gap's own text allows retention only as an explicit choice, which this
Decision supplies.

## Rejected alternatives

- Rename all three files with an all-caller cutover in one commit: rejected
  because it edits at least twelve citations across closed records and
  README for zero content gain, and risks a partially migrated citation
  graph inside this PR.
- Rename only the singly-cited files (I-000002, PLAN-000001): rejected as an
  inconsistent half-measure that leaves the high-traffic P-000001 slug
  contradicting its content.
- Edit record content to match the slugs: rejected because that would
  reinstate rejected claims (first-class ES256, publish/consume scope,
  full-parity promise) that D-000007, G-000010, and G-000008 corrections
  removed.

## Consequences

G-000012 moves to `accepted` with this Decision as its basis. Search or
navigation by filename can still surface superseded wording; readers must
consult the record's title, State, and body. The citations retained in
closed records stay byte-stable.
I-000003 carries the standard Invariant derived from this Decision, so the
binding citation rule is discoverable outside this rationale record.

## Revisit when

A bulk record-migration procedure exists that renames paths and rewrites all
callers atomically (for example at a future closure boundary), or the
records are restructured such that these paths lose their citation traffic.

## Provenance

Survey corrected in place on open PR #2 before any closing checkpoint (this
record was born in this PR; CodeRabbit thread PRRT_kwDOUlFWIM6kzMgg): counts
refreshed from 10 / 1 / 1 at head f21b6d3 to 12 / 2 / 2 at the then-current
assurance-branch head 6c098f3, now counting this Decision's own three
citations; the lower-bound-of-twelve statement is retained.
