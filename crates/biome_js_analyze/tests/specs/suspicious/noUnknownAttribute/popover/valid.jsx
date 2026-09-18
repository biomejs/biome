/* should not generate diagnostics */
<div>
	<button popoverTarget="my-popover" popoverTargetAction="toggle">
		Open Popover
	</button>

	<div id="my-popover" onBeforeToggle={this.onBeforeToggle} popover>
		Greetings, one and all!
	</div>
</div>;
