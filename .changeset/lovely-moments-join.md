---
"@biomejs/biome": patch
---

Added the rule [`noVueReservedProps`](https://biomejs.dev/linter/rules/no-vue-reserved-props/), resolves [#6309](https://github.com/biomejs/biome/issues/6309).

It prevents the use of reserved Vue prop names such as `key` and `ref` which can cause conflicts and unexpected behavior in Vue components.

##### Invalid example

```js
import {defineComponent} from 'vue';

export default defineComponent({
    props: [
        'ref',
        'key',
        'foo',
    ]
});
```

```vue
<script setup>
defineProps({
    ref: String,
    key: String,
    foo: String,
});
</script>
```

##### Valid examples

```js
import {defineComponent} from 'vue';

export default defineComponent({
    props: ['foo']
});
```

```vue
<script setup>
defineProps({ foo: String });
</script>
```
