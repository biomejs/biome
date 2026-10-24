<!-- should not generate diagnostics -->
<script setup lang="ts">
const SLOT_NAME = "computed";
defineSlots<{
    default(props: {}): any;
    [SLOT_NAME](props: {}): any;
    [name: `item-${string}`]: (props: { index: number }) => any;
}>();
</script>

<template>
    <slot />
    <slot name="item-1" />
    <slot name="computed" />
</template>
