<!-- should generate diagnostics -->
<template>
	<!-- div with onclick but no keyboard handler -->
	<div @click="handleClick()"></div>
	<div v-on:click="handleClick()"></div>

	<!-- span with onclick but no keyboard handler -->
	<span @click="handleClick()">text</span>

	<!-- <a> without href is not interactive -->
	<a @click="handleClick()">bare link</a>
</template>
