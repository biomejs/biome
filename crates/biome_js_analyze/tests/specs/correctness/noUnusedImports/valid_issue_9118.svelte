<!-- should not generate diagnostics -->
<script lang="ts">
	import MyElement, {
		type MyElementShape,
	} from './my-element.svelte';

	let myElement = $state<MyElementShape>();
</script>

<MyElement bind:this={myElement} />

