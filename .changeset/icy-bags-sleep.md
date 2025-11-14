---
"@biomejs/biome": patch
---

Added the nursery rule [`noSyncScripts`](https://biomejs.dev/linter/rules/no-sync-scripts/). Prevent the usage of synchronous scripts.

**Invalid:**

```jsx
<script src="https://third-party-script.js" />
```

**Valid:**

```jsx
<script src="https://third-party-script.js" async />
<script src="https://third-party-script.js" defer />
```
