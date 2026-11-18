---
"@biomejs/biome": patch
---

Fixed unnecessary language-server project rescans when an editor reports configuration or ignore-file changes inside force-ignored directories. Changes to active configuration files still reload the project.
