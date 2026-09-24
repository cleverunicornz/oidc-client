# oidc-client

`oidc-client` is a standalone Rust OpenID Connect relying-party library
carrying forward the [`openidconnect`](https://github.com/ramosbugs/openidconnect-rs)
4.0.1 baseline. It provides discovery, authorization-code requests with PKCE,
ID-token validation, UserInfo, refresh-token requests, and dynamic client
registration.

The baseline was imported from upstream commit
[b639b5d39eac6903238867aeb2b29326502e6b26](https://github.com/ramosbugs/openidconnect-rs/tree/b639b5d39eac6903238867aeb2b29326502e6b26)
(tag [`4.0.1`](https://github.com/ramosbugs/openidconnect-rs/releases/tag/4.0.1)).
The import's limited metadata, rename, and lint-remediation deltas are recorded
in [`situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md).

## Capabilities

- OpenID Connect discovery
- Authorization-code flow with PKCE
- ID-token verification for RS256/RS384/RS512, PS256/PS384/PS512, ES256, ES384,
  EdDSA (Ed25519), and HS256/HS384/HS512 (HS* for confidential clients holding
  the client secret)
- Signed UserInfo responses with an explicit algorithm allowlist (RS256-only
  default; other algorithms opt-in; HS* additionally requires a confidential
  client) and confidential-client support
- A single documented access-token-hash (`at_hash`) verification flow covering
  all supported signature families
- Refresh-token requests
- Dynamic client registration
- A deliberate break from upstream 4.x: `client_secret_expires_at` is now
  `Option<ClientSecretExpiration>` with explicit `NeverExpires` / `ExpiresAt`
  semantics.

## Status

The crate is version `4.1.0`, a maintained continuation of the
[`openidconnect`](https://github.com/ramosbugs/openidconnect-rs) 4.0.1 baseline.
ES256 (ECDSA P-256) ID-token verification is assured end-to-end under
[`situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md):
a full ES256-signed ID-token fixture exercises signature, issuer, audience,
nonce, and expiry validation through the public verifier, judged by
[`situation/oracles/O-000002-judge-es256-verification.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/oracles/O-000002-judge-es256-verification.md)
and witnessed by
[`situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md).
Canonical scope and evidence live in
[`situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md)
for the carried flows and
[`situation/promises/P-000003-published-on-crates-io-as-oidc-client.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/promises/P-000003-published-on-crates-io-as-oidc-client.md)
for the publication track. The state of every promise on this line, including
the newer hardening work, is recorded under
[`situation/`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/).

## Minimum supported Rust version

The minimum supported Rust version is **1.96** — the oldest toolchain verified
by recorded compilation evidence, retained in
[`situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md`](https://github.com/cleverunicornz/oidc-client/blob/main/situation/witnesses/P-000015/W-000018-msrv-1-96-compile-proof.md).
The dependency graph may resolve with older toolchains, and its true lower
floor may sit below 1.96, but that floor is unproven and not claimed:
supported-minimum claims come from compilation evidence, never from manifest
metadata alone.

## Attribution

This crate exists because of the work of David A. Ramos (ramosbugs) and
contributors on `openidconnect`. The upstream code is MIT-licensed; the
original [`LICENSE`](https://github.com/cleverunicornz/oidc-client/blob/main/LICENSE)
is preserved in this repository and
[`NOTICE.md`](https://github.com/cleverunicornz/oidc-client/blob/main/NOTICE.md)
carries the formal attribution.

## License

MIT — see [`LICENSE`](https://github.com/cleverunicornz/oidc-client/blob/main/LICENSE)
and [`NOTICE.md`](https://github.com/cleverunicornz/oidc-client/blob/main/NOTICE.md).
