<!-- should generate diagnostics -->
<script setup lang="ts">
const isNewSheetOpen = defineModel<boolean>("isNewSheetOpen");
const actuallyUnused = true;
</script>

<template>
	<button @click="() => (isNewSheetOpen = true)">button</button>
</template>
