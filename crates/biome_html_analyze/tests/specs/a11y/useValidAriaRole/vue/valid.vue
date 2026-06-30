<!-- should not generate diagnostics -->
<div role="button"></div>
<div role="switch" />
<div role></div>
<div></div>
<div baz />
<div role="button" />
<div role="switch row" />
<div />
<div role="tabpanel row" />
<div v-bind:role="roleValue" />
<div :role="roleValue" />
<!-- Static Vue bindings with valid roles should not generate diagnostics -->
<div :role="'button'" />
<div v-bind:role="'switch'" />
<div :role="'button row'" />
