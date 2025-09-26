---
"@biomejs/biome": minor
---

#### lineEnding has a new option 'auto', that is CRLF on Windows, and LF on non-Windows

lineEnding now has an option to match the operating system's expected
line-ending style, on Windows this will be CRLF and on macOS / Linux this will
be LF. This allows for cross-platform projects that use Biome to not have to
force one option or the other, which aligns better with Git's default behavior
on these platforms.

**Example usage:**
```json
{
  "formatter": {
    "lineEnding": "auto"
  }
}
```

```bash
biome format --line-ending auto
```
