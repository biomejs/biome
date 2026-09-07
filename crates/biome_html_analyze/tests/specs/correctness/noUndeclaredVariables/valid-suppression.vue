<!-- should not generate diagnostics -->
<template>
    <!-- biome-ignore lint/correctness/noUndeclaredVariables: directive is registered by a plugin -->
    <div v-rule-suppressed></div>
    <!-- biome-ignore lint/correctness: directive is registered by a plugin -->
    <div v-group-suppressed></div>
</template>
