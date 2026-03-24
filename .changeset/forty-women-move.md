---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUntrustedLicenses`](https://biomejs.dev/linter/rules/no-untrusted-licenses/). This rule disallows dependencies that ship with invalid licenses or licenses that don't meet the criteria of your project/organisation.

The rule has the following options:
- `allow`: a list of licenses that can be allowed. Useful to bypass possible invalid licenses from downstream dependencies.
- `deny`: a list of licenses that should trigger the rule. Useful to deny licenses that don't fit your project/organisation.
  When both `deny` and `allow` are provided, `deny` takes precedence.
- `requireOsiApproved`: whether the licenses need to be approved by the [Open Source Initiative](https://opensource.org/).
- `requireFsfLibre`: whether the licenses need to be approved by the [Free Software Foundation](https://www.gnu.org/licenses/license-list.html).
