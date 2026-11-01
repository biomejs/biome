---
"@biomejs/biome": patch
---

Fixed the location of Tailwind CSS diagnostics reported on unquoted object keys. In `clsx({ shadow: active })`, diagnostics now highlight `shadow` instead of starting one character later at `hadow:`.
