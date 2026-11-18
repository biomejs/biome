/* should generate diagnostics */
<>
	<button disabled>Save</button>
	<button disabled={condition}>Save</button>
	<button disabled={true}>Save</button>
	<button disabled="false">Save</button>
	<div role="button" disabled>Save</div>
	<input type="submit" disabled={condition} />
	<button disabled aria-disabled="true">Save</button>
	<button disabled /* comment */>Save</button>

	<input disabled />
	<input type="text" disabled={condition} />
	<input type="email" disabled={true} name="email" />
	<textarea disabled="disabled" />
	<textarea disabled /* comment */ />
	<textarea disabled={/* comment */ condition} />
	<input type="text" disabled readOnly />
	<input type="text" disabled readonly />

	<select disabled />
	<input type="checkbox" disabled={condition} />
	<select disabled aria-disabled="true" />

	<fieldset disabled />
</>
