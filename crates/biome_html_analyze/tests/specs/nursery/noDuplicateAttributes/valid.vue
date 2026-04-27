<!-- should not generate diagnostics -->

<template>
	<div foo :bar="bar" />

	<div class :class="a" />
	<div class :Class="a" />
	<div style :style="a" />
	<div style :Style="a" />

	<!-- dynamic arguments are ignored -->
	<div foo :[foo]="bar" />
	<div click @click="handler" />
	<div v-on:[evt]="a" @click="b" />
</template>
