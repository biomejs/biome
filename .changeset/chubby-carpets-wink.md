---
"@biomejs/biome": minor
---

Added new rule [useConsistentResponse](https://biomejs.dev/linter/rules/use-consistent-response) which suggests to use static [Response.json()](https://developer.mozilla.org/en-US/docs/Web/API/Response/json) and [Response.redirect()](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static) methods instead of `new Response` when possible.

Example:
```js
new Response(JSON.stringify({ value: 1 }));
Response.json({ value: 1 })
``` 
