<!-- should not generate diagnostics -->
<script setup lang="ts">
interface Slots {
    default(props: { msg: string }): any;
    footer: (props: {}) => any;
}
defineSlots<Slots>();
</script>

<template>
    <slot msg="hello" />
    <slot name="footer" />
</template>
