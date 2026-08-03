<!-- should not generate diagnostics -->
<style scoped>
.card {
	--local-color: blue;
}
</style>
<template>
	<div class="card" style="color: var(--local-color)"></div>
	<div :style="dynamicStyle"><span style="color: var(--dynamic-color)"></span></div>
	<div :style="dynamicStyle" style="color: var(--same-element-dynamic-color)"></div>
</template>

<style scoped>
:global(:root) {
	--global-color: green;
}
</style>
