<!-- should not generate diagnostics -->
<script setup lang="ts">
type Slots = {
    default(props: { msg: string }): any;
    footer(props: {}): any;
};
const slots = defineSlots<Slots>();
</script>

<template>
    <slot msg="hello" />
    <slot name="footer" />
</template>
