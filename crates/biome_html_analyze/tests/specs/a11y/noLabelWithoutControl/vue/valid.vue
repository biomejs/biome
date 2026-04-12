<!-- should not generate diagnostics -->

<!-- Basic valid: for + text -->
<label for="js_id">A label</label>

<!-- aria-label as accessible text -->
<label for="js_id" aria-label="A label"></label>

<!-- aria-labelledby as accessible text -->
<label for="js_id" aria-labelledby="some-id"></label>

<!-- Wraps an input with text content -->
<label>A label<input /></label>

<!-- Template expression counts as accessible text -->
<label>{{ labelText }}<input /></label>

<!-- PascalCase <Label> is a custom component — ignored entirely -->
<Label></Label>
<Label for="js_id"></Label>

<!-- Lowercase <input> matches as a control in Vue/Svelte/Astro -->
<label>A label<input /></label>
