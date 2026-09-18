<!-- should generate diagnostics -->
<!-- .ghost is not defined in any <style> block (scoped or global). -->
<template>
	<div class="ghost">Undefined class</div>
	<div class="title">Defined in scoped style — valid</div>
</template>

<style scoped>
	.title { font-size: 2rem; }
</style>
