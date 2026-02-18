<!-- should emit diagnostics -->

<script setup>
import { toRefs, reactive } from 'vue';
defineProps<{ toast: string; number?: number }>();
const obj = reactive({ toast: 'toast' });
const { toast } = toRefs(obj)
</script>

<template>
	<!-- Which `toast` this refers to is ambiguous, because defineProps makes the props available in the template area -->
	{{ toast }}
</template>
