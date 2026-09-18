<!-- should generate diagnostics -->
<div role="range"></div>
<div role="datepicker"></div>
<div role=""></div>
<div role="datepicker" />
<div role="unknown-invalid-role" />
<div role="tabpanel row foobar"></div>
<div role="doc-endnotes range"></div>
<!-- static Vue bindings with invalid roles should also be flagged -->
<div :role="'range'" />
<div v-bind:role="'datepicker'" />
<div :role="'unknown-invalid-role'" />
