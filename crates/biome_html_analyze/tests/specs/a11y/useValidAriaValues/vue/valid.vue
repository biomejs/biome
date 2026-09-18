<template>
<!-- should not generate diagnostics -->

<!-- valid boolean values -->
<div aria-hidden="true">content</div>

<!-- valid tristate -->
<span aria-checked="mixed">some text</span>

<!-- dynamic bindings are skipped (runtime expressions) -->
<div :aria-hidden="isHidden">content</div>
<div v-bind:aria-checked="checked">content</div>
</template>
