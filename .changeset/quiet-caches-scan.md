---
"@biomejs/biome": patch
---

Fixed the scanner's ignored directory names (built-in and configured) being matched against the project's ancestor directories. When the project lived inside a directory whose name matched one of them, nested `.gitignore` files were not honored. These names now only match path components inside the project root.
