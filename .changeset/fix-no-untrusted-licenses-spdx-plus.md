---
"@biomejs/biome": patch
---

Fixed [#13428](https://github.com/biomejs/biome/issues/13428), where [`noUntrustedLicenses`](https://biomejs.dev/linter/rules/no-untrusted-licenses/) reported valid SPDX license expressions with the `+` ("or later") operator (such as `Apache-2.0+` and `MPL-2.0+`) as untrusted.
