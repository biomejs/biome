<!-- should not generate diagnostics: Vue v-bind shorthand (:aria-*) bindings satisfy required aria props -->
<template>
	<span role="checkbox" :aria-checked="isChecked"></span>
	<span role="radio" :aria-checked="true"></span>
	<span role="switch" :aria-checked="dynamicValue"></span>
	<span role="heading" :aria-level="level"></span>
	<span role="option" :aria-selected="selected"></span>
	<span
		role="slider"
		:aria-valuemax="max"
		:aria-valuemin="min"
		:aria-valuenow="current"
	></span>
	<span role="meter" :aria-valuenow="progress"></span>
</template>
