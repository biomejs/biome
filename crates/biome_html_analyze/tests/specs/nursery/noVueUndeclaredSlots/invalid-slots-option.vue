<!-- should generate diagnostics -->
<script lang="ts">
import { defineComponent, type SlotsType } from "vue";

export default defineComponent({
    slots: Object as SlotsType<{
        default: { msg: string };
    }>,
});
</script>

<template>
    <slot name="footer" />
</template>
