<!-- should not generate diagnostics -->
<template>
	<button :disabled="false">Save</button>
	<button :disabled="(false)">Save</button>
	<button :disabled="null">Save</button>
	<button :disabled="(null)">Save</button>
	<button :disabled="undefined">Save</button>
	<button :disabled="(undefined)">Save</button>
	<button :disabled="false /* static */">Save</button>
	<button :disabled="condition" v-bind="props">Save</button>
	<div :role="role" disabled>Save</div>
	<div role="switch button" disabled>Save</div>
	<div role="button" v-bind="props" disabled>Save</div>
	<input :type="type" disabled />
	<input type="submit" v-bind="props" disabled />
	<select role="button" disabled />
	<textarea role="button" disabled />
	<fieldset role="button" disabled />
	<optgroup role="button" disabled />
	<option role="button" disabled />
	<Button disabled />
	<my-button role="button" disabled />
	<button :[disabled]="condition">Save</button>
	<button v-bind="props">Save</button>
</template>
