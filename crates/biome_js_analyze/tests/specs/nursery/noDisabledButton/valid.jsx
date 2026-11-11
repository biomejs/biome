/* should not generate diagnostics */
<>
	<button disabled={false}>Save</button>
	<button disabled={null}>Save</button>
	<button disabled={undefined}>Save</button>
	<button disabled {...props}>Save</button>
	<button {...props}>Save</button>

	<input type={type} disabled />
	<input disabled />
	<input type="text" role="button" disabled />
	<input type="submit" {...props} disabled />

	<div role={role} disabled>Save</div>
	<div role="switch button" disabled>Save</div>
	<div role="button" {...props} disabled>Save</div>
	<div role="button" disabled {...props}>Save</div>

	<select role="button" disabled />
	<textarea role="button" disabled />
	<fieldset role="button" disabled />
	<optgroup role="button" disabled />
	<option role="button" disabled />

	<Button disabled />
	<Form.Button disabled />
	<form:button disabled />
	<my-button role="button" disabled />
</>
