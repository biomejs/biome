---
"@biomejs/biome": patch
---

Added the rule [`noVueReservedKeys`](https://biomejs.dev/linter/rules/no-vue-reserved-keys/), which prevents the use of reserved Vue keys.

It prevents the use of Vue reserved keys such as those starting with `$` (like `$el`, `$data`, `$props`) and keys starting with `_` in data properties, which can cause conflicts and unexpected behavior in Vue components.

##### Invalid example

```vue
<script>
export default {
    data: {
        $el: '',
        _foo: 'bar',
    },
};
</script>
```

```vue
<script>
export default {
    computed: {
        $data() {
            return this.someData;
        },
    },
};
</script>
```

##### Valid examples

```vue
<script>
export default {
    data() {
        return {
            message: 'Hello Vue!',
            count: 0,
        };
    },
};
</script>
```

```vue
<script>
export default {
    computed: {
        displayMessage() {
            return this.message;
        },
    },
};
</script>
```
