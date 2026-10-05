<!-- should not generate diagnostics -->
<template>
<div v-pre></div>

<!-- ensure other directives do not trigger this rule -->
<div v-text="foo"></div>
</template>
