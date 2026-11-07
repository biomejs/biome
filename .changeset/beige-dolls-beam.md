---
"@biomejs/biome": patch
---

Fixed GritQL code snippets matching JSX text. A snippet that parses as code on its own, such as `` `process.env = $value` ``, no longer matches JSX text that reads the same, such as `<p>process.env = value</p>`. Snippets that don't parse as a single piece of code, such as `` `Hello $name` ``, still match JSX text.
