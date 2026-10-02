---
"@biomejs/biome": patch
---

Fixed false positives in [`noUnknownAttribute`](https://biomejs.dev/linter/rules/no-unknown-attribute/) for React 19 transition event handlers. `onTransitionCancel`, `onTransitionRun`, `onTransitionStart`, and their capture variants are recognized when the React dependency range allows React 19 or later.
