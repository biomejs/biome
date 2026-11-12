---
"@biomejs/biome": patch
---

Fixed [#13107](https://github.com/biomejs/biome/issues/13107): the unsafe fix of [`noDistractingElements`](https://biomejs.dev/linter/rules/no-distracting-elements/) now removes the whole element, including its closing tag and children, instead of only the opening tag. The fix is no longer offered when removing the element would produce invalid code, such as `const Ticker = () => <marquee>News</marquee>;`.
