# G-000020 — Citation-stability Decision has no Invariant

## State

closed

## Gap

D-000011 states a standing repository citation rule but creates or links no
Invariant carrying that rule. The Decision contract separates rationale from
binding rules and requires a resulting Invariant when a Decision produces one.

## Relevance

D-000011 and G-000012 are changed records in closure run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`. The concern is
whether the selected slug-stability policy is represented in the record class
that repository work consults for binding rules; it does not reopen the
underlying choice to retain the three paths.

## Evidence

Validator observation for run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` at fixed reviewed
head `4a05b4511348644c65352a5a0373c11370bd1fd3`:

- `situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md`
  decides that record content governs meaning, that the slug is not an
  authority, and that "No record may be cited for a proposition by slug
  alone." Its Consequences repeat the standing reading rule.
- D-000011 contains no link to an Invariant, and the current
  `situation/invariants/` records contain no rule about slugs or citation
  coordinates.
- `situation/decisions/AGENTS.md` says a Decision states why and an Invariant
  states the resulting rule; when a Decision produces a binding rule, the
  Invariant is written and links the Decision as its Basis.
- `situation/invariants/AGENTS.md` defines an Invariant as the binding
  repository rule and provides the Basis link for a Decision-derived rule.
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22):
  situation/invariants/I-000003-record-content-governs-citation-propositions.md
  exists with Priority `standard`, the citation rule, and D-000011 as its
  Basis; D-000011 references it in its Decision and Consequences sections.
  Those records froze with the links at closing checkpoint `f78f641`; this
  lane added only the additive G-000012 reference.

## Impact

The rationale and Gap disposition are retained, but the selected standing rule
is absent from the namespace designated for binding repository rules. A reader
consulting applicable Invariants can miss the policy while the Decision uses
normative language as a second authority.

## Resolution

closed — situation/invariants/I-000003-record-content-governs-citation-propositions.md
carries the slug-stability rule as a `standard` Invariant with D-000011 as
its Basis: a citation supports a proposition through the cited record's
title, State, and body, and the filename slug identifies the coordinate.
D-000011 links the Invariant in its Decision and Consequences sections, and
G-000012 links it additively in References, so the standing rule now lives
in the binding-rule namespace.

## References

- situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md
- situation/gaps/G-000012-record-path-slugs-retain-rejected-claims.md
- situation/decisions/AGENTS.md
- situation/invariants/AGENTS.md
- situation/invariants/I-000003-record-content-governs-citation-propositions.md —
  the resulting binding rule.
