---
"@biomejs/biome": patch
---

Fixed [#10531](https://github.com/biomejs/biome/issues/10531): the JavaScript formatter now gives the same result on the first and second run for method chains like `reply.code(409).send({ ... })` that don't fit on one line. Previously, the first run put each call on its own line and broke the object, and a second run moved the chain back onto one line.

```diff
- reply
-   .code(409)
-   .send({
-     ok: false,
-     error: "User with this email already exists here",
-     code: "DUPLICATE_EMAIL",
-   });
+ reply.code(409).send({
+   ok: false,
+   error: "User with this email already exists here",
+   code: "DUPLICATE_EMAIL",
+ });
```
