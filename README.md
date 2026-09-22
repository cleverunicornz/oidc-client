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
in [`situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md`](situation/decisions/D-000004-import-fidelity-rules-for-the-openidconnect-4-0-1-import.md).

## Status

The manifest declares version `4.1.0`, but the crate is not yet published on
crates.io. The carried source implements ES256 (ECDSA P-256) signature
verification with EC JWKs. That behavior is assured end-to-end: a full
ES256-signed ID-token fixture exercises signature, issuer, audience, nonce,
and expiry validation through the public verifier, witnessed under
[`situation/oracles/O-000002-judge-es256-verification.md`](situation/oracles/O-000002-judge-es256-verification.md)
by
[`situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md`](situation/witnesses/P-000002/W-000003-es256-id-token-fixture-public-verifier.md).
Canonical scope and evidence live in
[`situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md`](situation/promises/P-000001-full-oidc-rp-flows-work-identically-to-openidconnect-4-0-1.md),
[`situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md`](situation/promises/P-000002-es256-ecdsa-p-256-id-token-verification.md),
and [`situation/promises/P-000003-published-on-crates-io-as-oidc-client.md`](situation/promises/P-000003-published-on-crates-io-as-oidc-client.md).

## Attribution

This crate exists because of the work of David A. Ramos (ramosbugs) and
contributors on `openidconnect`. The upstream code is MIT-licensed; the
original [`LICENSE`](LICENSE) is preserved in this repository and
[`NOTICE.md`](NOTICE.md) carries the formal attribution.

## License

MIT — see [`LICENSE`](LICENSE) and [`NOTICE.md`](NOTICE.md).
