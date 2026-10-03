<!-- should generate diagnostics -->
<script setup lang="ts">
import { foo } from "./foo";
import type { Bar } from "./bar";
</script>

<template>
	<div>{{ foo }}</div>
</template>
