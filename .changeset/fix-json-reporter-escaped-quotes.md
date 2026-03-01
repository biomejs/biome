---
"@biomejs/biome": patch
---

Fixed JSON reporter escaping of special characters in diagnostic messages. The JSON reporter now properly escapes double quotes, backslashes, and control characters in error messages and advice text, preventing invalid JSON output when diagnostics contain these characters.
