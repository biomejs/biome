<template>
	<div v-foo:bar.baz="5"></div>
	<div :bar.baz="5"></div>
	<div @bar.baz="5"></div>
</template>