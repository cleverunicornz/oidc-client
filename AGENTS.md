<bedrock-repository>
## oidc-client

- Identity: A standalone Rust OpenID Connect relying-party library carrying the attributed openidconnect 4.0.1 baseline and producing client APIs for discovery, authorization, token verification, UserInfo, refresh, and dynamic registration.
- Ownership: `OWNED`.
- Phase and implementation map: `situation/context.md`.
- Critical invariants: none.
- Verification: Assured — [P-000005](situation/promises/P-000005-configured-ci-gate-route.md) is judged by [O-000006](situation/oracles/O-000006-judge-configured-ci-gate-route.md); dispatch `gh workflow run ci.yml --ref bank2/assurance`, whose successful result is retained by [W-000004](situation/witnesses/P-000005/W-000004-configured-ci-fleet-run.md), and repository gate claims cite the resulting workflow-run URL.
- Tool priority: organization defaults.
- Donor boundary: `92946acf13d02a67caab38cb64444a902217fae4` (the BACKPORT opening checkpoint at which the trigger tree became historical donor material).
</bedrock-repository>
