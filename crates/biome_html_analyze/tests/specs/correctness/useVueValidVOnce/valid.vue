<!-- should not generate diagnostics -->
<template>
<div v-once></div>

<!-- ensure other directives do not trigger this rule -->
<div v-text="foo"></div>
</template>
