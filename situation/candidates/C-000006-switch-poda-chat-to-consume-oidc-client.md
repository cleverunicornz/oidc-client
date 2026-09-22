# C-000006 — Switch Poda Chat to consume oidc-client

## State

proposed

## Candidate

In Poda Chat, replace the openidconnect dependency with the published
`oidc-client` package and migrate the Rust import path where needed.

## Origin

The standalone-public-crate decision
`situation/decisions/D-000001-create-oidc-client-as-a-standalone-public-crate-not-a-vendored-fork.md`
identifies Poda Chat as the intended consumer after publication.

## Why consider it

The downstream application needs a normal Cargo dependency on the maintained
continuation before it can benefit from its release and algorithm support.

## Qualification questions

- Is an appropriate `oidc-client` release published and resolvable?
- Which Poda Chat import paths and dependency-policy entries change?
- Does Poda Chat's own qualification evidence show no behavioral regression?

## Candidate approaches

- Migrate Poda Chat to a published crates.io release through its own pull
  request and Bedrock closure.
- Continue consuming upstream openidconnect until the release boundary is
  ready.

## Disposition

none — this repository does not own the Poda Chat change or its evidence.
