---
"@biomejs/biome": patch
---

The HTML formatter now picks the quote character for an attribute by counting the quotes in the value rather than looking only for a double quote. `&apos;` and `&quot;` count as the characters they stand for, and only the character that ends up as the delimiter stays escaped:

```diff
- <div title='123 &apos;&quot; 456'></div>
+ <div title="123 '&quot; 456"></div>
```

Entities that are not quotes, such as `&amp;` or `&#39;`, are left exactly as written.
