---
"@biomejs/biome": patch
---

Fixed an issue where explicitly targeting a file under `node_modules` (e.g. `biome lint --vcs-use-ignore-file=false node_modules/some-pkg/index.js`) reported a confusing `internalError/fs` diagnostic ("does not exist in the workspace") instead of actually linting the file, even though it was just opened successfully.
