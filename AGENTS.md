<bedrock-repository>
## oidc-client

- Identity: A standalone Rust OpenID Connect relying-party library carrying the attributed openidconnect 4.0.1 baseline and producing client APIs for discovery, authorization, token verification, UserInfo, refresh, and dynamic registration.
- Ownership: `OWNED`.
- Phase and implementation map: `situation/context.md`.
- Critical invariants: [I-000004](situation/invariants/I-000004-public-secure-material-boundary.md) — This repository is public. Secure material — our own known vulnerabilities from the security vault, their details, exploitability and affected code paths — is never represented in it: not in code, comments, commits, branches, issues, pull requests, reviews or comments. Public CVE and CWE references are fine. A fix lands normally, with a Promise that describes the property the code keeps, never the vulnerability.
- Verification: Assured — [P-000017](situation/promises/P-000017-ci-route-supersedes-pull-request-runs.md) (superseding P-000005) is judged by [O-000027](situation/oracles/O-000027-judge-ci-route-supersedes-pull-request-runs.md); a same-repository pull request run (opened, reopened, ready_for_review) or `gh workflow run ci.yml --ref <branch>` runs the gate, whose successful result is retained by [W-000040](situation/witnesses/P-000017/W-000040-ci-route-supersedes-pull-request-runs.md), and repository gate claims cite the resulting workflow-run URL.
- Tool priority: organization defaults.
- Donor boundary: `92946acf13d02a67caab38cb64444a902217fae4` (the BACKPORT opening checkpoint at which the trigger tree became historical donor material).
</bedrock-repository>
