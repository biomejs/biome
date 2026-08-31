---
"@biomejs/biome": patch
---

Fixed an issue where the Biome Language Server would start with logging level set to debug. This would cause logs to grow exponentially in long sessions.
