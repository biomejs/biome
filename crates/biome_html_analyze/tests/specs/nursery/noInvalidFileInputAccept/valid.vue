<!-- should not generate diagnostics -->
<template>
	<input :type="type" accept="image/jpg" />
	<input type="file" :accept="allowedTypes" />
	<input type="file" accept="image/jpg" :accept="allowedTypes" />
	<input type="file" accept="image/jpg" v-bind="props" />
	<input type="file" accept="image/jpg" :[name]="value" />
	<input type="file" accept="image/jpg" v-bind:[name]="value" />
</template>
