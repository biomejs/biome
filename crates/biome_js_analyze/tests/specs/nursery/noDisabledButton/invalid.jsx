/* should generate diagnostics */
<>
	<button disabled>Save</button>
	<button role="switch" disabled={condition}>Save</button>
	<button disabled="false">Save</button>
	<button {...props} disabled>Save</button>

	<input type="button" disabled />
	<input type="submit" disabled />
	<input type="reset" disabled />
	<input type="image" disabled />
	<input type="SUBMIT" disabled />
	<input type={"button"} disabled />
	<input {...props} type="submit" disabled />

	<div role="button" disabled>Save</div>
	<div role="unknown button" disabled>Save</div>
	<a role={"button"} disabled>Save</a>
	<div {...props} role="button" disabled>Save</div>
</>
