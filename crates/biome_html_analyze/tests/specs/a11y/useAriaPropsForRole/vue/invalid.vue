<!-- should generate diagnostics -->
<template>
	<span role="checkbox"></span>
	<span role="combobox" aria-controls="true"></span>
	<span role="heading"></span>
	<span role="menuitemcheckbox"></span>
	<span role="menuitemradio"></span>
	<span role="meter"></span>
	<span role="radio"></span>
	<span role="scrollbar" aria-valuemin="0"></span>
	<span role="scrollbar" aria-controls="123"></span>
	<span role="scrollbar" aria-valuemin="0" aria-valuenow="100"></span>
	<span role="scrollbar" aria-valuemin="0" aria-valuemax="100" aria-orientation="horizontal"></span>
	<span role="separator"></span>
	<span role="slider" aria-valuemin="0"></span>
	<span role="slider" aria-valuemax="100"></span>
	<span role="slider" aria-valuemin="0" aria-valuemax="100"></span>
	<span role="switch"></span>

	<!-- :role="'checkbox'" requires aria-checked -->
	<span :role="'checkbox'"></span>
	<span v-bind:role="'checkbox'"></span>
	<!-- :role="'heading'" requires aria-level -->
	<div :role="'heading'"></div>
	<!-- :role="'option'" requires aria-selected -->
	<span :role="'option'"></span>
</template>
