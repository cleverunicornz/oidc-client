# C-000006 — Switch Poda Chat to consume oidc-client

## State

proposed

## What

Replace the openidconnect 4.0.1 dependency in Poda Chat with the published
oidc-client crate. This is a Cargo.toml change plus any import-path
renames.

## Approach

1. In poda-chat Cargo.toml: replace openidconnect = "4.0.1" with
   oidc-client = "x.y.z"
2. Update all use statements from openidconnect:: to oidc_client::
3. Update deny.toml: remove the RUSTSEC-2023-0071 ignore if resolved
4. Verify: cargo build, cargo clippy, full test suite, serial DB suite all
   pass
5. This change lands through the normal Poda Chat PR/Bedrock process

## Dependencies

C-000005 (the crate must be published first)

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000006
(project Status: Todo).
