<!-- should generate diagnostics -->
<template>
	<!-- ghost-class is referenced here but never defined in any <style> block. -->
	<div class="ghost-class">Undefined class</div>

	<!-- header is defined in the global <style> block, so it's valid. -->
	<div class="header">Valid header</div>
</template>

<style>
	.header { font-size: 2rem; }
</style>

<style scoped>
	/* scoped is local — .local-only applies to this component's own elements
	   but is NOT the class being tested here. */
	.local-only { color: red; }
</style>
