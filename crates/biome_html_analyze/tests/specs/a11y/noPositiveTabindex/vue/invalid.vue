<!-- should generate diagnostics -->
<div tabindex="1"></div>
<div tabindex="5"></div>
<button tabindex="2"></button>
<a href="#" tabindex="10"></a>
<input tabindex="99" />
<!-- static Vue bindings with positive tabindex should also be flagged -->
<div :tabindex="'1'"></div>
<div v-bind:tabindex="'5'"></div>
<button :tabindex="'2'"></button>
