---
"@biomejs/biome": patch
---

Added the rule [`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/), which prevents duplicate keys in Vue component definitions.

This rule prevents the use of duplicate keys across different Vue component options such as `props`, `data`, `computed`, `methods`, and `setup`. Even if keys don't conflict in the script tag, they may cause issues in the template since Vue allows direct access to these keys.

##### Invalid examples

```vue
<script>
export default {
    props: ['foo'],
    data() {
        return {
            foo: 'bar'
        };
    }
};
</script>
```

```vue
<script>
export default {
    data() {
        return {
            message: 'hello'
        };
    },
    methods: {
        message() {
            console.log('duplicate key');
        }
    }
};
</script>
```

```vue
<script>
export default {
    computed: {
        count() {
            return this.value * 2;
        }
    },
    methods: {
        count() {
            this.value++;
        }
    }
};
</script>
```

##### Valid examples

```vue
<script>
export default {
    props: ['foo'],
    data() {
        return {
            bar: 'baz'
        };
    },
    methods: {
        handleClick() {
            console.log('unique key');
        }
    }
};
</script>
```

```vue
<script>
export default {
    computed: {
        displayMessage() {
            return this.message.toUpperCase();
        }
    },
    methods: {
        clearMessage() {
            this.message = '';
        }
    }
};
</script>
```
