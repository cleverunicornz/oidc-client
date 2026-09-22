# G-000023 — Multi-file edit section was misapplied

## State

closed

## Gap

The record-editing tool accepted a multi-file patch but placed the second file
section and its operations as literal text in the first file after a whole-file
replacement. The intended second-file update was not applied in that operation.

## Relevance

Bedrock record corrections use line-anchored edits to preserve historical
bytes. An accepted cross-file patch that changes a different record than its
section header names can corrupt the correction surface before ordinary record
review detects it.

## Evidence

- During correction run
  `20260922T160102Z-2225beb0f70ca521911ef82a7bad12ec04d91a52`, a patch intended
  to update `situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md`
  and `situation/promises/P-000005-configured-ci-gate-route.md` placed the
  second section header and its content in the Witness file instead.
- The malformed content was detected by an exact re-read before publication,
  removed, and the Promise update was reapplied as a single-file edit.
- The harness defect was reported through the designated tool-report channel.
- Bookkeeping observation (PR #3 branch `bank3/bookkeeping`, 2026-09-22): the
  current bytes verify clean — W-000004 contains only its own Witness sections
  (Promise, Oracle, Result, Head, Observed, Evidence, Oracle legs) with no
  foreign section header or promise text, and P-000005 carries the intended
  update: the `ci` job's fork-guard eligibility boundary in Scope and the
  per-run witness boundaries in State evidence.

## Impact

A future multi-file record correction could silently modify the wrong record
while leaving the intended target unchanged. No malformed content from this
observation was committed.

## Resolution

closed — detected-and-corrected. The misfiled section was never committed:
an exact re-read caught it before publication, the malformed content was
removed, and the Promise update was reapplied as a single-file edit. The
current bytes of W-000004 and P-000005 verify clean, and the incident remains
recorded in the Evidence above together with the tool-report channel
notification. No record corruption from this observation exists in the
repository's history.

## References

- situation/AGENTS.md
- situation/gaps/AGENTS.md
