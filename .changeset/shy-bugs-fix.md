---
"@biomejs/biome": patch
---
Fix FileTooLarge diagnostic severity from Information to Warning, so that --error-on-warnings correctly exits with a non-zero code when a file exceeds files.maxSize.
