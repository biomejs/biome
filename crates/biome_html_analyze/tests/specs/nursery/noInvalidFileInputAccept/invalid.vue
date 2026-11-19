<!-- should generate diagnostics -->
<template>
	<input type="file" accept="image/jpg" />
	<input v-bind="props" type="file" accept="image/jpg" />
	<input :[name]="value" type="file" accept="image/jpg" />
	<input v-bind:[name]="value" type="file" accept="image/jpg" />
</template>
