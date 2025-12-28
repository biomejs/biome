---
"@biomejs/biome": patch

Added the nursery rule ['noBeforeInteractiveScriptOutsideDocument'](https://biomejs.dev/linter/rules/no-before-interactive-script-outside-document/) to Next.js domain.
This rule prevents usage of `next/script`'s `beforeInteractive` strategy outside of `pages/_document.js`.
