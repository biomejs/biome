---
"@biomejs/biome": patch
---

Added support for the `corner-shape` family of CSS properties and the `superellipse()`/`squircle()` value functions, so [`noUnknownProperty`](https://biomejs.dev/linter/rules/no-unknown-property/) and [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function/) no longer flag them as unknown.

New known properties: `corner-shape`, `corner-block-end-shape`, `corner-block-start-shape`, `corner-bottom-left-shape`, `corner-bottom-right-shape`, `corner-bottom-shape`, `corner-end-end-shape`, `corner-end-start-shape`, `corner-inline-end-shape`, `corner-inline-start-shape`, `corner-left-shape`, `corner-right-shape`, `corner-start-end-shape`, `corner-start-start-shape`, `corner-top-left-shape`, `corner-top-right-shape`, `corner-top-shape`.

New known value functions: `superellipse()`, `squircle()`.
