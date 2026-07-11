<!-- should generate diagnostics -->
<input autofocus />
<input v-bind:autofocus="autofocusValue" />
<input :autofocus="autofocusValue" />
<input autofocus />
<input autofocus="true" />
<textarea autofocus>content</textarea>
<button autofocus>Submit</button>
<select autofocus><option>a</option></select>

<!-- autofocus on dialog/popover container itself is invalid -->
<dialog autofocus>content</dialog>
<div popover autofocus>content</div>
