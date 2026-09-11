---
"@biomejs/biome": patch
---

Fixed [#11716](https://github.com/biomejs/biome/issues/11716): the [`noUnknownAttribute`](https://biomejs.dev/linter/rules/no-unknown-attribute/) rule now accepts fullscreen event handlers, the `credentialless` iframe property, and the SVG `maskType` property when the React dependency range allows React 19.3 or later. The `credentialless` and `maskType` properties are restricted to `<iframe>` and `<mask>` elements, respectively.
