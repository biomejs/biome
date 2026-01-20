<!-- should generate diagnostics -->

<template>
	<div foo :foo="bar" />
	<div v-bind:foo="a" :foo="b" />

	<!-- event handlers are ignored -->
	<div v-on:click="a" @click="b" />

	<div class class :class="a" />
	<div style style :style="a" />

	<!-- dynamic arguments are ignored -->
	<div foo :[foo]="a" />
	<div v-on:[evt]="a" @click="b" />
</template>
