---
"@biomejs/biome": patch
---

Fix a bug where `--config-path` accepted configuration files with unsupported extensions. Now only `.json` and `.jsonc` are accepted, and an error is raised otherwise.
