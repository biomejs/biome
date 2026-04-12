<!-- should generate diagnostics -->

<!-- Empty label: no text, no control -->
<label></label>

<!-- Has text but no control association -->
<label>A label</label>

<!-- Has for attribute but no accessible text -->
<label for="js_id"></label>

<!-- Uppercase <INPUT> is NOT matched as a control in Vue (case-sensitive) -->
<label>A label<INPUT /></label>

<!-- PascalCase <Input> is treated as a custom component, not a control -->
<label>A label<Input /></label>
