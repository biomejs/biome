---
"@biomejs/biome": patch
---

Fixed [#13257](https://github.com/biomejs/biome/issues/13257): untitled VS Code documents now use their existing workspace project, retain it across workspace-folder removal and re-addition, avoid creating projects from non-file URIs, and preserve URI extensions for language detection.
