---
"@biomejs/biome": patch
---

Fix UTF-8 byte indexing panic with CRLF line endings in string_utils

Fixed panic in `string_utils.rs` when processing files with Windows (CRLF) line endings. Added bounds checking and UTF-8 character boundary validation to prevent unsafe byte slicing operations.

Fixes #9180
