---
"@biomejs/biome": patch
---

Fixed [#12628](https://github.com/biomejs/biome/issues/12628): [`noPlaywrightUselessAwait`](https://biomejs.dev/linter/rules/no-playwright-useless-await/) no longer reports `await` on non-Playwright receivers calling methods such as `.or()`. The following code is now valid:

```ts
const { data } = await query.is('deleted_at', null).or('a.eq.1,b.eq.2');
```
