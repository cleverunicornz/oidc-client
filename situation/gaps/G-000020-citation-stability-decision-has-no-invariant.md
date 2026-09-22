# G-000020 — Citation-stability Decision has no Invariant

## State

open

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

## Impact

The rationale and Gap disposition are retained, but the selected standing rule
is absent from the namespace designated for binding repository rules. A reader
consulting applicable Invariants can miss the policy while the Decision uses
normative language as a second authority.

## Resolution

none

## References

- situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md
- situation/gaps/G-000012-record-path-slugs-retain-rejected-claims.md
- situation/decisions/AGENTS.md
- situation/invariants/AGENTS.md
