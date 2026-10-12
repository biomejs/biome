---
"@biomejs/biome": patch
---

The HTML formatter now formats Vue `@event` handlers that contain statements. A single expression stays on one line without a semicolon, and multiple statements are written on lines of their own inside the quotes, like Prettier does.

```diff
- <button @click="count  +=   1"></button>
- <button @click="save( );close( )"></button>
+ <button @click="count += 1"></button>
+ <button
+   @click="
+     save();
+     close();
+   "
+ ></button>
```

The parser also accepts handlers that start with a statement, such as `@click="if (ok) save()"`. The formatter adds the `;` that Vue needs to compile them as the body of a function:

```diff
- <button @click="if (ok) save()"></button>
+ <button @click="if (ok) save();"></button>
```

Handlers that Vue compiles as the body of a function keep their semicolons with `"semicolons": "asNeeded"`, because Vue compiles a handler without a `;` as an expression.
