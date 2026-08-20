---
"@biomejs/biome": patch
---

Fixed Astro attribute names being split on `:` and `.` inside an expression, such as `{x && <button x-on:keyup.enter={go} client:load.foo />}`.
