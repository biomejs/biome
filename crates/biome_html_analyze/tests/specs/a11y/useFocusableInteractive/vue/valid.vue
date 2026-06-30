<!-- should not generate diagnostics -->
<div>
	<div role="button" tabindex="0"></div>

	<div role="menu">
		<div role="menuitem" tabindex="0">
			Open
		</div>
		<div role="menuitem" tabindex="-1">
			Save
		</div>
		<div role="menuitem" tabindex="-1">
			Close
		</div>
	</div>
	<button></button>
	<div role="h1"></div>
	<!-- static Vue role bindings with interactive roles + tabindex are valid -->
	<div :role="'button'" tabindex="0"></div>
	<div v-bind:role="'tab'" tabindex="0"></div>
</div>
