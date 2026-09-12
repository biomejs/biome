<!-- should generate diagnostics -->
<script setup>
function configure() {
    const vNestedBinding = {};
    defineOptions({ directives: { nestedOption: {} } });
}
</script>

<template>
    <div v-nested-binding></div>
    <div v-nested-option></div>
</template>
