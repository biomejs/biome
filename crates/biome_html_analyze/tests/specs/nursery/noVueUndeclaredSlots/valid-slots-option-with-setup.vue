<!-- should not generate diagnostics -->
<script lang="ts">
import type { SlotsType } from "vue";

export default {
    slots: Object as SlotsType<{
        default: { msg: string };
    }>,
};
</script>

<script setup lang="ts">
const message = "hello";
</script>

<template>
    <slot :msg="message" />
</template>
