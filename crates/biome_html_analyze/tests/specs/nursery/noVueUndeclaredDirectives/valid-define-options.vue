<!-- should not generate diagnostics -->
<script setup>
defineOptions({
    directives: {
        definedWithOptions: {},
    },
});
</script>

<template>
    <div v-defined-with-options></div>
</template>
