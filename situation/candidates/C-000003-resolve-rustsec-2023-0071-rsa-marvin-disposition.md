# C-000003 — Resolve RUSTSEC-2023-0071 (rsa Marvin) disposition

## State

promoted

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

D-000003 and D-000005 resolved the affected-path and recorded-disposition
questions. A configured fleet run of the dependency-policy step is an
assurance question for P-000005/O-000006; P-000004/O-000005 separately judge
the selected local configuration.

## Candidate approaches

- Upgrade to an unaffected `rsa` release when one exists.
- Use exact, recorded advisory ignores with explicit revisit conditions.
- Globally relax advisory severity or leave the gate red.

## Disposition

Promoted by
`situation/decisions/D-000008-promote-dependency-policy-contract.md` into
P-000004 and O-000005. D-000003 and D-000005 retain the underlying scoped
advisory decisions.
