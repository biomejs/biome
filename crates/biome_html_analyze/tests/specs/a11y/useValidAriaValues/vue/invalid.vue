<template>
<!-- should generate diagnostics -->

<!-- invalid boolean value -->
<div aria-hidden="yes">content</div>

<!-- invalid tristate value -->
<span aria-checked="test">some text</span>

<!-- static Vue bindings with invalid aria values should also be flagged -->
<div :aria-hidden="'yes'">content</div>
<span v-bind:aria-checked="'invalid'">some text</span>
</template>
