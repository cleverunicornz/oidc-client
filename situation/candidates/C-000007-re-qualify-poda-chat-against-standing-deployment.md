# C-000007 — Re-qualify Poda Chat against its standing deployment

## State

proposed

## Candidate

After Poda Chat adopts a published `oidc-client`, rerun its deployment-shape
qualification to determine whether the dependency swap preserves its observed
authentication behavior.

## Origin

The planned downstream swap in
`situation/candidates/C-000006-switch-poda-chat-to-consume-oidc-client.md`
and the retained Poda Chat qualification context prompted this possibility.

## Why consider it

A downstream Cargo migration is not evidence of behavior parity against the
running provider. The consumer repository owns the relevant deployment,
credentials, and result records.

## Qualification questions

- Has Poda Chat completed the dependency migration?
- Which standing-deployment journeys, key rotation paths, and token algorithms
  are within its release scope?
- Does that repository retain a complete oracle and witness for the swap?

## Candidate approaches

- Run Poda Chat's existing deployment-shape qualification after the migration.
- Defer the migration until its deployment evidence can be collected.

## Disposition

none — this repository records the dependency relationship but does not own the
Poda Chat deployment or its witnesses.
