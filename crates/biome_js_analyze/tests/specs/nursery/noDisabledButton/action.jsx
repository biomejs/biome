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
</>
