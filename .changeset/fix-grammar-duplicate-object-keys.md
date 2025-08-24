---
"@biomejs/biome": patch
"@biomejs/backend-jsonrpc": patch
---

fix(noDuplicateObjectKeys): correct grammatical error in JSON diagnostic message

Fixed missing "is" in the diagnostic message for the noDuplicateObjectKeys rule when applied to JSON files. The message now correctly reads "This is where a duplicated key was declared again." instead of "This where a duplicated key was declared again."
