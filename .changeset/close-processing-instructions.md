---
"@biomejs/biome": patch
---

Fixed the HTML parser not closing processing instructions such as `<?xml version="1.0"?>` in Astro, Vue and Angular files. The `?>` was read as part of an attribute name, so everything after it ended up in the end-of-file token. Instructions with no space before `?>`, such as `<?xml?>` and `<?foo bar?>`, now close in every HTML-based language, plain HTML included.
