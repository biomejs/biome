<template>
	<!-- v-if/else chain -->
	<div v-if="count === 1">one</div>
	<div v-else-if="count === 2">two</div>
	<div v-else>other</div>

	<!-- v-bind longhand/shorthand -->
	<div v-bind:title="title"></div>
	<div :id="elemId"></div>
	<input :disabled>
	<div v-bind:[attrName]="attrValue"></div>
	<div :[dataKey]="dataVal"></div>

	<!-- v-on longhand/shorthand, dynamic args, modifiers -->
	<button v-on:click="onClick"></button>
	<input @input="onInput">
	<div v-on:[evt]="onEvent"></div>
	<div @[evt]="onEventShort"></div>
	<button @keyup.enter.stop.prevent="onKey"></button>

	<!-- v-model and modifiers -->
	<input v-model="text">
	<input v-model.lazy.number="count">

	<!-- v-show -->
	<div v-show="isShown"></div>

	<!-- mix with v-for -->
	<ul>
		<li v-for="(item, index) in items" :key="index">Item</li>
	</ul>
</template>