# G-000012 — Record path slugs retain rejected claims

## State

open

## Gap

Several records corrected during the closure retain path slugs that describe
the superseded or rejected wording rather than the record now stored at that
path. The filenames remain syntactically valid kebab case, but their semantic
labels can lead discovery to the opposite of the current content.

## Relevance

Repository-relative paths are the protocol's stable citation coordinates and
appear throughout the affected lineage. Search and human navigation commonly
surface the filename before the title or state.

## Evidence

- Validator observation for run
  `20260922T105519Z-37afd430f1f61a2faa2fb8481bb6d8933837626c` at reviewed head
  `5a8580f3394f853c995e7ccb1148fe86c040321d`.
- `situation/invariants/I-000002-es256-is-a-first-class-supported-algorithm.md`
  now says ES256 must not be represented as first-class while unassured.
- `situation/plans/active/PLAN-000001-import-es256-publish-consume.md` now has
  the title and scope "Establish imported OIDC behavior and gate evidence" and
  no longer contains publication or consumption work.
- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
  now makes the narrower implemented-baseline claim and explicitly leaves
  live-flow parity unassured.

## Impact

A path-only citation or search result can repeat rejected first-class,
publish/consume, or full-flow-parity wording and obscure the corrected record
boundary. Renaming would require an all-caller clean cutover; leaving the paths
may be acceptable only as an explicit stability choice.

## Resolution

none

## References

- `situation/decisions/D-000007-bound-es256-support-claims-to-evidence.md`
- `situation/plans/active/PLAN-000001-import-es256-publish-consume.md`
- `situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`
