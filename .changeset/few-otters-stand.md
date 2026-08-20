---
"@biomejs/biome": minor
---

Implemented the ability to load extended configurations from parents, grandparents and so on. The resolution is limited to a depth level of ten.

In other words, configuration files that are shipped as npm packages can now extend other configuration files (local or other npm packages).
