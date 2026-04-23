---
"@biomejs/biome": minor
---

Ported [`useValidAriaValues`](https://biomejs.dev/linter/rules/use-valid-aria-values/) to HTML. Biome now validates static `aria-*` attribute values in HTML elements against WAI-ARIA types, catching invalid values such as `aria-hidden="yes"`.
