---
"@biomejs/biome": patch
---

Fixed a stack overflow in `useDestructuring` when analyzing cyclic TypeScript type aliases such as `type A = B; type B = A;`.
