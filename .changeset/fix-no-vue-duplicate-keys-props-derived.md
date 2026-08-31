---
"@biomejs/biome": patch
---

Fixed a false positive in [`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/) where a `<script setup>` variable initialized from `props` was reported as a duplicate of the prop it derives from. Biome now exempts any variable whose initializer references `props`, instead of only recognizing `defineProps()` and `toRefs(props)`, matching `eslint-plugin-vue`'s [vue/no-dupe-keys](https://eslint.vuejs.org/rules/no-dupe-keys) rule.

For example, Biome no longer reports `foo` below as a duplicate key:

```vue
<script setup>
import { toRef } from 'vue';
const props = defineProps(['foo']);
const foo = toRef(props, 'foo');
</script>
```
