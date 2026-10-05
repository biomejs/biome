<!-- should not generate diagnostics -->
<template>
    <!-- biome-ignore lint/nursery/noVueUndeclaredDirectives: directive is registered by a plugin -->
    <div v-rule-suppressed></div>
    <!-- biome-ignore lint/nursery: directive is registered by a plugin -->
    <div v-group-suppressed></div>
</template>
