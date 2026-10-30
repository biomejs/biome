<!-- should not generate diagnostics -->
<script setup>
let count = 0;
console.log(count);
count = 1;
</script>

<template>
	<p>{{ count }}</p>
</template>
