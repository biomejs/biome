<template>
	<!-- Chain v-bind and v-on with dynamic arguments and modifiers in various orders -->
	<div id="root">
		<!-- Mixed v-bind (longhand/shorthand), dynamic args, boolean, and v-on with dynamic args and modifiers -->
		<button
			v-bind:[dataKey].camel="buttonTitle"
			:[ariaKey].camel="ariaVal"
			:tabindex="'0'"
			v-on:[evt].once.passive="handle"
			@[evt].capture.self="handle2"
			@click.stop.prevent="clicked"
		>
			Click
		</button>

		<!-- Dynamic + modifiers on both v-bind and v-on, plus boolean dynamic bind without value -->
		<input
			:[placeholderKey]="placeholder"
			@keyup.enter.exact="onEnter"
			v-on:[keyEvt].capture="onKeyEvt"
		/>

		<!-- Multiple chained directives with dynamic names on a component -->
		<component-x
			v-bind:[propName].camel="propVal"
			v-bind:[styleKey].camel="styleVal"
			v-on:[eventName].once="onEvent"
			@[rawEvt].self="onRaw"
			:is="componentType"
		></component-x>

		<!-- Interleaved longhand/shorthand with dynamic args and string literal values -->
		<a
			v-bind:[hrefKey].camel="url"
			@[clickEvt].prevent="follow"
			:target="'_blank'"
			:rel="'noopener'"
		>Link</a>

		<!-- Dense chaining to exercise attribute list termination across many tokens -->
		<div
			:[dataA].camel="valA"
			:[dataB]="valB"
			v-on:[hoverEvt].once.passive="onHover"
			@click.exact.stop.prevent="onClickExact"
			@[keydownEvt].capture.self="onKeyDown"
		>Dense</div>
	</div>
</template>
