<template>
	<!-- boolean longhand -->
	<input v-bind:disabled />

	<!-- boolean shorthand -->
	<button :disabled>Click</button>

	<!-- dynamic argument longhand -->
	<div v-bind:[dataKey]="dataValue"></div>

	<!-- dynamic argument shorthand -->
	<div :[attrName]="attrValue"></div>

	<!-- mixed: dynamic argument with modifier -->
	<component v-bind:[propName].camel="val"></component>

	<!-- multiple v-bind on the same element -->
	<img :src="imgSrc" v-bind:[altKey]="altText" />

	<!-- dynamic argument without initializer (boolean) -->
	<div :[flagKey]></div>
</template>