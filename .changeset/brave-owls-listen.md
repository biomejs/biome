---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueConsistentEventHyphenation`](https://biomejs.dev/linter/rules/use-vue-consistent-event-hyphenation/), a port of `vue/v-on-event-hyphenation`. It enforces a consistent hyphenation style for event names in `v-on` directives on custom components. By default, hyphenated names are required, so `<MyComponent @customEvent="handler" />` is reported.
