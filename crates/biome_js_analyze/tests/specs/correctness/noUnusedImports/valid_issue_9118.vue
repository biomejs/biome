<!-- should not generate diagnostics -->
<script lang="ts">
	import MyElement, {
		type MyElementShape,
	} from './my-element.vue';

	let myElement: MyElementShape =  {};
</script>

<template>
	<MyElement />
</template>

