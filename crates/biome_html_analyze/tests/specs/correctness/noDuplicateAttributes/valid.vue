<!-- should not generate diagnostics -->

<template>
	<div foo :bar="bar" />

	<div class :class="a" />
	<div style :style="a" />

	<!-- dynamic arguments are ignored -->
	<div foo :[foo]="bar" />
	<div click @click="handler" />
	<div v-on:[evt]="a" @click="b" />
</template>
