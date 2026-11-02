<!-- should not generate diagnostics -->
<script setup>
const message = "hello";
</script>

<template>
    <slot :msg="message" />
    <slot name="footer" />
</template>
