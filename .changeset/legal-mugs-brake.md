---
"@biomejs/biome": patch
---

Fixed #8907: `useExhaustiveDependencies` now correctly recognizes stable hook results (like `useState` setters and `useRef` values) when declared with `let`.
