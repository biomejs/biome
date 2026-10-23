---
"@biomejs/biome": patch
---

Fixed [#12505](https://github.com/biomejs/biome/issues/12505): [`useDomNodeTextContent`](https://biomejs.dev/linter/rules/use-dom-node-text-content/) no longer reports `innerText` on values whose type annotation shows they aren't DOM nodes. The following code is now valid:

```tsx
const Link = ({ innerText }: { innerText: string }) => <a>{innerText}</a>;
```
