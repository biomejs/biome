<!-- should not generate diagnostics -->
<script>
const defineComponent = (options) => options;

export default defineComponent({
    directives: {
        definedWithComponent: {},
    },
});
</script>

<template>
    <div v-defined-with-component></div>
</template>
