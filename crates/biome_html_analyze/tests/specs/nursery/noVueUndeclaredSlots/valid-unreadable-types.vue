<!-- should not generate diagnostics -->
<script setup lang="ts">
import type { BaseSlots } from "./slots";

interface Slots extends BaseSlots {
    default(props: { msg: string }): any;
}
defineSlots<Slots>();
</script>

<template>
    <slot name="from-base-slots" />
</template>
