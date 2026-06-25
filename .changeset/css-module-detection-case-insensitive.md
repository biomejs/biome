---
"@biomejs/biome": patch
---

Fixed CSS Module detection so files are recognized regardless of extension case. Previously a path like `Foo.MODULE.CSS` was classified as plain CSS instead of a CSS Module because the well-known suffix check was case-sensitive, mirroring the existing case-insensitive handling in `JsFileSource`.
