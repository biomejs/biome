---
"@biomejs/biome": minor
---

#### lineEnding has a new option `auto`

The option `lineEnding` now has a variant called `auto` to match the operating system's expected
line-ending style: on Windows, this will be CRLF (`\r\n`), and on macOS / Linux, this will
be LF (`\n`).

This allows for cross-platform projects that use Biome not to have to
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
