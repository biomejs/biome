<!-- should generate diagnostics -->
<script setup lang="ts">
type Slots = {
    footer: (props: { msg: string }) => any;
};
defineSlots<Slots>();
</script>

<template>
    <slot />
</template>
