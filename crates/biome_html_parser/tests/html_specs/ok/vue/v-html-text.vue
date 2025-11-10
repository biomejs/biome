<template>
	<div v-html="rawHtml"></div>
	<span v-text="message"></span>
	<p v-text="'static text'"></p>
	<div v-html="count + ' items'"></div>
	<div v-bind:title="title"></div>
	<div :id="elemId"></div>
	<input :disabled />
	<div v-bind:[attrName]="attrValue"></div>
	<div :[dataKey]="dataVal"></div>
	<button v-on:click="onClick"></button>
	<input @input="onInput" />
	<div v-on:[evt]="onEvent"></div>
	<button @keyup.enter.stop.prevent="onKey"></button>
	<input v-model="text" />
	<input v-model.lazy.number="count" />
	<div v-show="isShown"></div>
	<ul>
		<li v-for="(item, index) in items" :key="index">Item</li>
	</ul>
</template>