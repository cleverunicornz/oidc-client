# G-000017 — Stale final-head labels in retained evidence

## State

open

## Gap

Current records call earlier commits the branch's "final head" even though the
fixed reviewed head is later. The cited runs and survey remain pinned to exact
commits, but the unqualified final-head labels no longer describe the branch
history presented by those same records.

## Relevance

The labels occur in changed CI-assurance and citation-survey records reviewed
for closure run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`. Precise temporal
labels matter when readers decide which repository bytes a retained run or
survey actually observed. This concern is distinct from whether the underlying
manual-dispatch run passed O-000006.

## Evidence

Validator observation for run
`20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` at fixed reviewed
head `4a05b4511348644c65352a5a0373c11370bd1fd3`:

- `situation/promises/P-000005-configured-ci-gate-route.md` calls the green
  dispatch at `5dbfc4b46204ca7c36c39283103e4efbe88e1c56` the branch's final
  head, and
  `situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md` repeats
  that label. Both records also retain the run's exact head and successful
  result.
- `situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md`
  calls `6c098f3d6cbab582530e621f612f0aca39750b87` the final head for its
  citation survey and provenance. The Decision itself was subsequently
  changed at `874940da5073bd59271515af094703f9cf361202`, before the fixed
  reviewed head.
- The later commits are record and closure-reconciliation changes; this
  observation does not assert that they alter the Rust behavior exercised by
  the run at `5dbfc4b46204ca7c36c39283103e4efbe88e1c56`.

## Impact

A reader may interpret "final head" as evidence that the run or survey observed
all bytes at the reviewed head, rather than an earlier exact commit. The pinned
commit identifiers preserve the real boundary, so the effect on the underlying
CI result and citation counts is uncertain; the concern is the competing
unqualified label.

## Resolution

none

## References

- situation/promises/P-000005-configured-ci-gate-route.md
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
