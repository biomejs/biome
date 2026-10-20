---
"@biomejs/biome": patch
---

Fixed `noDescendingSpecificity` reporting a false positive for selectors nested under different parent selectors. The rule now compares the resolved selector target, so `&:hover` nested in two unrelated rules is no longer treated as the same target.
