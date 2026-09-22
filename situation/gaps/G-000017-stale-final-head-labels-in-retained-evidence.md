# G-000017 — Stale final-head labels in retained evidence

## State

closed

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
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22): an
  exact search for "final head" over `situation/` at this branch matches only
  this Gap's own observation text. P-000005's State evidence names the
  primary witness at its exact head
  `7fe8265c8166e16ea4da77b5722a35503fa67665` with no final-head label;
  W-000004 declares that same exact head; D-000011 says the survey counts
  were refreshed "at the then-current assurance-branch head `6c098f3`". The
  relabeling landed in forward commits of run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52` before its
  closing checkpoint.

## Impact

A reader may interpret "final head" as evidence that the run or survey observed
all bytes at the reviewed head, rather than an earlier exact commit. The pinned
commit identifiers preserve the real boundary, so the effect on the underlying
CI result and citation counts is uncertain; the concern is the competing
unqualified label.

## Resolution

closed — the unqualified labels were replaced with temporally exact
references: P-000005 names the primary PASS witness and its exact head
`7fe8265c8166e16ea4da77b5722a35503fa67665`, W-000004 declares that exact
head for its bounded run, and D-000011 pins its survey to the exact heads
`f21b6d3` and `6c098f3` as "then-current" rather than final. The runs and
the survey remain pinned to their exact commits, so the temporal boundary
of every retained observation is readable from the record itself.

## References

- situation/promises/P-000005-configured-ci-gate-route.md
- situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md
- situation/decisions/D-000011-record-path-slugs-remain-stable-citation-coordinates.md
- situation/gaps/G-000004-no-assured-ci-witness-route.md
