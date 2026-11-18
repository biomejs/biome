<!-- should generate diagnostics -->
<script setup lang="ts">
defineSlots<{
    default(props: { msg: string }): any;
    "item-row": (props: { index: number }) => any;
}>();
</script>

<template>
    <div>
        <slot name="footer" />
        <slot name="itemRow" />
        <slot :name="'literal'" />
        <slot :name="`template`" />
        <template v-if="true">
            <slot name="nested"></slot>
        </template>
    </div>
</template>
