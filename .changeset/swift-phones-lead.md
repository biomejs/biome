---
"@biomejs/biome": patch
---

Fixed [#12088](https://github.com/biomejs/biome/issues/12088): false positives in [`noUnknownAttribute`](https://biomejs.dev/linter/rules/no-unknown-attribute/) for React 19 transition event handlers. `onTransitionCancel`, `onTransitionRun`, `onTransitionStart`, and their capture variants are recognized when the React dependency range allows React 19 or later.
