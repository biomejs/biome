<!-- should generate diagnostics -->
<button disabled={condition}>Save</button>
<button {disabled}>Save</button>
<button disabled={true}>Save</button>
<button disabled="false">Save</button>
<div role="button" disabled={condition}>Save</div>
<button disabled aria-disabled="true">Save</button>
<input type="text" disabled={condition} />
<input type="text" {disabled} />
<textarea disabled={true}></textarea>
<textarea disabled="false"></textarea>
<input type="text" disabled readonly />
<select {disabled}></select>
<input type="checkbox" disabled={condition} />
<fieldset disabled={condition}></fieldset>
