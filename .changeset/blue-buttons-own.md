---
"@biomejs/biome": minor
---

Added support for Cursor files. When Biome sees a Cursor JSON file, it will parse it with comments enabled and trailing commas enabled:
- `$PROJECT/.cursor/`
- `%APPDATA%\Cursor\User\` on Windows
- `~/Library/Application Support/Cursor/User/` on macOS
- `~/.config/Cursor/User/` on Linux
