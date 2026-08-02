<!-- should not generate diagnostics -->
<style scoped>
.card {
	--local-color: blue;
}
</style>
<template>
	<div class="card" style="color: var(--local-color)"></div>
</template>

<style scoped>
:global(:root) {
	--global-color: green;
}
</style>
