# C-000003 — Resolve RUSTSEC-2023-0071 (rsa Marvin) disposition

## State

qualifying

## Candidate

Establish a narrow, evidence-based dependency-policy disposition for the
unpatched `rsa` Marvin advisory and the related dev-path informational
advisories.

## Origin

The retained cargo-deny bring-up failure in
`situation/witnesses/evidence/W-000001/deny-check.log`, the RustSec advisory,
and the imported crate's `rsa` dependency surfaced this candidate.

## Why consider it

An unexplained ignore would weaken dependency policy, while a permanently red
gate would not truthfully report the dependency posture. The import's public
private-signing wrapper also prevents an overly broad verification-only claim.

## Qualification questions

- Is an unaffected or patched `rsa` release available?
- Which affected paths are production-reachable versus dev-only?
- Does the scoped configuration run on the configured CI route without
  relaxing unrelated advisories?

The first two questions inform D-000003 and D-000005. The CI-route question
remains open pending a retained fleet-run Witness.

## Candidate approaches

- Upgrade to an unaffected `rsa` release when one exists.
- Use exact, recorded advisory ignores with explicit revisit conditions.
- Globally relax advisory severity or leave the gate red.

## Disposition

Qualification continues under
`situation/plans/active/PLAN-000001-import-es256-publish-consume.md`.
`situation/decisions/D-000003-rustsec-2023-0071-rsa-disposition-scoped-ignore.md`
and
`situation/decisions/D-000005-unmaintained-dev-path-advisory-ignores.md`
record the current configuration choices; no Candidate promotion occurs until
the resulting operational gate behavior has its own justified Promise/Oracle
lineage or the candidate is otherwise settled.
