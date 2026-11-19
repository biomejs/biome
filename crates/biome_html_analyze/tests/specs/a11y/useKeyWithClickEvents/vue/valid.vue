<!-- should not generate diagnostics -->

<template>
	<!-- onclick with keyboard handlers -->
	<div @click="handleClick()" @keydown="handleKeyDown()"></div>
	<div v-on:click="handleClick()" v-on:keydown="handleKeyDown()"></div>

	<!-- inherently keyboard-accessible elements -->
	<button @click="handleClick()">Submit</button>
	<a @click="handleClick()" href="https://example.com">Link</a>

	<!-- hidden from screen reader -->
	<div @click="handleClick()" aria-hidden="true">content</div>

	<!-- PascalCase elements are custom components — skip -->
	<MyComponent @click="handleClick()">content</MyComponent>
</template>
