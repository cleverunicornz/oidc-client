# PLAN-000001 — Import, ES256, publish, consume

## Purpose

Take the openidconnect 4.0.1 code, add ES256 verification, publish as
oidc-client on crates.io, and switch Poda Chat to consume it.

## Candidates

- situation/candidates/C-000001-import-openidconnect-v4-0-1-code-with-attribution.md
- situation/candidates/C-000002-add-es256-verification-via-p256-crate.md
- situation/candidates/C-000003-resolve-rustsec-2023-0071-rsa-marvin-disposition.md
- situation/candidates/C-000004-ci-pipeline-fmt-clippy-test-audit.md
- situation/candidates/C-000005-publish-to-crates-io.md
- situation/candidates/C-000006-switch-poda-chat-to-consume-oidc-client.md
- situation/candidates/C-000007-re-qualify-poda-chat-against-standing-deployment.md

## Promises

- situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md
- situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md
- situation/promises/P-000003-published-on-crates-io-as-oidc-client.md

## Dependencies

- C-000001 blocks everything (need the code first)
- C-000002 depends on C-000001 (add ES256 to the imported code)
- C-000005 depends on C-000002, C-000003, C-000004 (publish a complete
  crate)
- C-000006 depends on C-000005 (consume the published crate)
- C-000007 depends on C-000006 (qualify the swap)

## Completion

Completes when all promises P-000001..003 are assured with witnesses and
Poda Chat is running against the new crate with no behavioral regression.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item PLAN-000001
(project Status: Todo).
