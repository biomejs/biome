---
"@biomejs/biome": patch
---

Fixed [#8518](https://github.com/biomejs/biome/issues/8518), where globally excluded files in a monorepo were still being processed when using `"extends": "//"`.

When a package-level configuration extends the root configuration with `"extends": "//"`, glob patterns (such as those in `files.includes`) are now correctly resolved relative to the project root directory, instead of the current workspace directory.
