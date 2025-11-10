<template>
	<!-- Basic v-model on native elements -->
	<input v-model="text" />
	<textarea v-model="message"></textarea>

	<!-- Modifiers on native inputs -->
	<input v-model.lazy="textLazy" />
	<input v-model.number="qty" />
	<input v-model.trim="name" />
	<input v-model.lazy.trim.number="complex" />

	<!-- v-model on custom components (default model) -->
	<custom-input v-model="value" />
	<my-input v-model="raw" @input="onInput" />

	<!-- Named v-models on components -->
	<date-picker v-model:value="date" />
	<custom-select v-model:selected="selected" />
	<child v-model:visible="isVisible" />
	<child v-model:format.trim.number="format" />

	<!-- Dynamic argument for v-model on components -->
	<component-a v-model:[propName].trim="propVal" />
	<component-b v-model:[getProp()].number="dyn" />
	<component-c v-model:[computedKey].lazy="dynLazy" />

	<!-- Native controls often used with v-model -->
	<select v-model="selectedOption">
		<option value="a">A</option>
		<option :value="someVal">B</option>
	</select>

	<input type="checkbox" v-model="flags" />
	<input type="radio" value="one" v-model="picked" />
	<input type="radio" value="two" v-model="picked" />

	<!-- Grouping and nested usage to exercise attribute list ends -->
	<div class="wrapper">
		<child v-model:open="isOpen" />
	</div>
</template>