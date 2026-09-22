# C-000003 — Resolve RUSTSEC-2023-0071 (rsa Marvin) disposition

## State

proposed

## What

The upstream crate depends on the rsa crate for RSA verification.
RUSTSEC-2023-0071 (the Marvin timing side-channel) affects the rsa crate
with no fixed release. Poda Chat currently ignores this advisory in
deny.toml with the reason: verification-only usage.

## Approach

1. Check if a fixed rsa release is now available (the advisory was
   published 2023; check current rsa crate versions)
2. If fixed: bump the rsa dependency to the fixed version
3. If NOT fixed: scope the disposition properly in this crate: document
   that the crate uses rsa for signature VERIFICATION only (public key
   operations), and the Marvin attack targets private-key operations
   (signing/decryption), making the advisory not applicable to
   verification-only consumers
4. Add a cargo-deny or cargo-audit configuration that properly scopes this

## Dependencies

situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000003
(project Status: Todo).
