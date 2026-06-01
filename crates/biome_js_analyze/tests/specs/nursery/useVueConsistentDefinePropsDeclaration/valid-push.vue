<!-- should not generate diagnostics -->
<script setup lang="ts">
import { ref } from "vue";

const myArray = ref<string[]>([]);

const selectModel = (x: string) => {
	myArray.value.push(x);
};
</script>

<template>
</template>
