/* should not generate diagnostics */
<div>
	<div role="button" tabindex={0} />
	<div role="button" tabindex={-1} />
	<div role="button" tabindex="0" />

	<div role="menu">
		<div role="menuitem" tabindex="0">
			Open
		</div>
		<div role="menuitem" tabindex="-1">
			Close
		</div>
	</div>
</div>;
