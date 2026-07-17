<!-- should generate diagnostics -->
<div>
	<div role="button"></div>
	<div role="tab"></div>
	<!-- static Vue role bindings with interactive roles but missing tabindex should also be flagged -->
	<div :role="'button'"></div>
	<div v-bind:role="'tab'"></div>
</div>
