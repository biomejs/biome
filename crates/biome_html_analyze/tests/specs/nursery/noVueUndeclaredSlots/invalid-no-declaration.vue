<!-- should generate diagnostics -->
<script setup lang="ts">
const message = "hello";
</script>

<template>
    <div>
        <slot />
        <slot name="header" :message="message" />
    </div>
</template>
