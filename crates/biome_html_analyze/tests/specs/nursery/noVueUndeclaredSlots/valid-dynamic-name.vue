<!-- should not generate diagnostics -->
<script setup lang="ts">
defineSlots<{
    default(props: {}): any;
}>();
const slotName = "footer";
</script>

<template>
    <slot :name="slotName" />
</template>
