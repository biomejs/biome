---
"@biomejs/biome": patch
---

Fixed Biome reporting parse errors for comments and trailing commas in Zed's global settings file (`~/.config/zed/settings.json`) on macOS and Windows. Biome's scanner resolved Zed's config directory using platform conventions, but Zed uses `~/.config/zed` on macOS and `%APPDATA%\Zed` on Windows, so the file is now correctly recognized as JSONC.
