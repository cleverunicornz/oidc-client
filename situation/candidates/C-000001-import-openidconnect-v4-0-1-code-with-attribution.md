# C-000001 — Import openidconnect 4.0.1 code with attribution

## State

promoted

## Candidate

Create a standalone `oidc-client` crate by importing the exact
openidconnect 4.0.1 surface with its MIT attribution and only the
predeclared continuity deltas.

## Origin

cleverunicornz Project #20 materialized on 2026-09-22, verified against the
pin retained in `situation/references/R-000001-upstream-pin.md`.

## Why consider it

The repository needs a public continuation rather than vendoring a Rust OIDC
client into Poda Chat. The upstream pin supplies an existing API and test
surface, while its MIT terms permit a credited continuation.

## Qualification questions

- Can the exact upstream pin, license, and import surface be identified?
- Can crate renames and newer-toolchain remediation remain mechanically
  classified instead of changing behavior?
- Can attribution remain visible in the standalone crate?

These questions were settled by D-000004 and the retained import evidence.

## Candidate approaches

- Import the pin into a new standalone repository with attribution.
- Use a GitHub fork of the upstream project.
- Vendor the code into Poda Chat.

## Disposition

Promoted by
`situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`
into P-000001 and the historical O-000001 rule. The corrected in-scope rule
is O-000004; its assurance remains separate from this promotion.
