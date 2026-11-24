---
"@biomejs/biome": patch
---

[#13384](https://github.com/biomejs/biome/issues/13384): Fixed the opt-in Svelte formatter changing whitespace and line endings inside malformed or unsupported embedded code. Biome also preserves `{@debug}` tags with invalid names, unparsed `then` and `catch` bindings, `each` headers with invalid item or index bindings, and malformed attribute values including empty expressions. If embedded code in a delimited block header fails to parse, the header stays unchanged while its body and branches can still format. An argument-free unknown `{@name}` tag or an `{@html}`, `{@render}`, or `{@const}` tag with a missing expression also stays unchanged without preventing formatting of independent markup. Unsupported syntax inside a delimited preprocessed template no longer prevents formatting independent markup. A spread on a root script tag leaves its body and `generics` value (including braced values) untouched because the language is uncertain. An unambiguous `lang="ts"` on an empty root script still enables TypeScript in `generics` and template expressions. If the closing delimiter of a block, expression, script, style, or preprocessed template cannot be verified, formatting stops without rewriting the file, even inside a format suppression.

```diff
-<div bind:{if}></div>
+<div bind:if={if}></div>
-<p>{}</p>
+<p>{   }</p>
```
