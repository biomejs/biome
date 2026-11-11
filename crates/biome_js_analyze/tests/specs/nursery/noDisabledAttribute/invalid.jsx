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

	<input disabled />
	<input type="text" disabled />
	<input type="EMAIL" disabled={condition} />
	<input type="date" disabled />
	<input type="number" disabled />
	<input type="unknown" disabled />
	<input type="text" role="button" disabled />
	<input {...props} type="text" disabled />
	<textarea disabled />
	<textarea role="button" disabled={condition} />

	<input type="checkbox" disabled />
	<input type="radio" disabled />
	<input type="range" disabled />
	<input type="color" disabled />
	<input type="FILE" disabled />
	<select disabled />
	<select role="button" disabled={condition} />

	<fieldset disabled><input type="text" /></fieldset>
	<fieldset role="button" disabled={condition} />

	<div role="button" disabled>Save</div>
	<div role="unknown button" disabled>Save</div>
	<a role={"button"} disabled>Save</a>
	<div {...props} role="button" disabled>Save</div>
</>
