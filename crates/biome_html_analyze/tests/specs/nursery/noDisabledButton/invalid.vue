<!-- should generate diagnostics -->
<script setup>
const undefined = true;
</script>
<template>
	<button :disabled="condition">Save</button>
	<button :disabled>Save</button>
	<button v-bind:disabled="condition">Save</button>
	<button v-bind:disabled>Save</button>
	<button :disabled="true">Save</button>
	<button :disabled="undefined">Save</button>
	<button disabled="false">Save</button>
	<button :disabled.prop="condition">Save</button>
	<input :type="'submit'" :disabled="condition" />
	<input :type="`submit`" disabled />
	<div role="button" :disabled="condition">Save</div>
	<div :role="'button'" disabled>Save</div>
	<div :role="`button`" disabled>Save</div>
</template>
