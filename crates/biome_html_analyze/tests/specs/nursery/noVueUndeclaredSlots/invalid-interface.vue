<!-- should generate diagnostics -->
<script setup lang="ts">
interface Slots {
    default(props: { msg: string }): any;
}
defineSlots<Slots>();
</script>

<template>
    <slot name="footer" />
</template>
