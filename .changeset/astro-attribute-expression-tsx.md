---
"@biomejs/biome": patch
---

Fixed Astro attribute expressions rejecting TypeScript and JSX syntax that is accepted in text expressions.

```astro
<Component icon={<Icon />} count={total as number} onSelect={(e: Event) => e} />
```
