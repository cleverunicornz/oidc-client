# G-000023 — Multi-file edit section was misapplied

## State

open

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

## Impact

A future multi-file record correction could silently modify the wrong record
while leaving the intended target unchanged. No malformed content from this
observation was committed.

## Resolution

none

## References

- situation/AGENTS.md
- situation/gaps/AGENTS.md
