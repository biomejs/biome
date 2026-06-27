<!-- should not generate diagnostics -->
<div></div>
<div tabindex="-1"></div>
<div v-bind:tabindex="tabindexValue"></div>
<div :tabindex="tabindexValue"></div>
<button tabindex="0"></button>
<a href="#" tabindex="0"></a>
<div role="button" tabindex="0"></div>
<article tabindex="-1"></article>
