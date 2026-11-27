<!-- should generate diagnostics -->
<template>
	<button :disabled="condition">Save</button>
	<button :disabled>Save</button>
	<button v-bind:disabled="condition">Save</button>
	<button v-bind:disabled>Save</button>
	<button :disabled="true">Save</button>
	<button :disabled="(true)">Save</button>
	<button :disabled="`enabled`">Save</button>
	<button :disabled="'false'">Save</button>
	<div role="button" :disabled="condition">Save</div>
	<button :disabled.prop="condition">Save</button>
	<button :disabled="condition" :aria-disabled="existing">Save</button>

	<input type="text" :disabled="condition" />
	<input type="text" :disabled />
	<textarea v-bind:disabled="condition" />
	<textarea v-bind:disabled />
	<input type="text" :disabled="true" name="email" />
	<input type="text" disabled />
	<input type="text" :disabled.prop="condition" />
	<input type="text" :disabled="condition" :readonly="existing" />

	<select :disabled="condition" />
	<input type="checkbox" :disabled="true" />

	<fieldset :disabled="condition" />
</template>
