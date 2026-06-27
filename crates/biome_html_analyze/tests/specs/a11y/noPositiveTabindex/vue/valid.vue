<!-- should not generate diagnostics -->
<div tabindex="0"></div>
<div v-bind:tabindex="tabindexValue"></div>
<div :tabindex="tabindexValue"></div>
<div tabindex="-1"></div>
<button tabindex="0"></button>
<a href="#" tabindex="-1"></a>
<input tabindex="0" />
<div></div>
