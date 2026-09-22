# C-000001 — Import openidconnect v4.0.1 code with attribution

## State

proposed

## What

Import the complete openidconnect v4.0.1 source into this repository.
Preserve the MIT license (the upstream tag is MIT-only — see R-000001). Add
a NOTICE or README section crediting the original authors (ramosbugs /
David Ramos and contributors). Do NOT use a GitHub fork — this is a fresh
import.

## Approach

1. Clone https://github.com/ramosbugs/openidconnect-rs at the 4.0.1 tag
   (pinned in R-000001)
2. Copy the faithful-import surface (41 files per R-000001: src/, tests,
   examples, build config; excludes .github/)
3. Rename the crate from openidconnect to oidc-client in Cargo.toml and all
   internal references
4. Carry forward the MIT LICENSE file with its copyright line (the upstream
   tag ships a single MIT LICENSE and no Apache license file)
5. Add a NOTICE.md or README section crediting upstream authors
6. Update the repository URL and crate metadata
7. Verify: cargo build, cargo test all pass on the imported code

## Dependencies

None — this is the first step.

## Provenance

Materialized 2026-09-22 from cleverunicornz Project #20 item C-000001
(project Status: Todo). Corrected against evidence: the project text says
"MIT OR Apache-2.0" and "Add LICENSE-MIT and LICENSE-APACHE files"; the
Phase A upstream-facts scout verified at pin b639b5d that the tag is
MIT-only with a single LICENSE file (R-000001).
