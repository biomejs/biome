<template>
	<div :bar='foo("x")'></div>
	<div :bar="'it\'s'"></div>
	<div v-for="x in ['a',b]">{{ x }}</div>
	<div v-for='x in ["a",b]'>{{ x }}</div>
	<div v-for='x in [`say "hi"`]'>{{ x }}</div>
</template>
