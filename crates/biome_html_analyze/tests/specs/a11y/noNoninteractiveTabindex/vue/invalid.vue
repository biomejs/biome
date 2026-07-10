<!-- should generate diagnostics -->
<div tabindex="0"></div>
<div role="article" tabindex="0"></div>
<article tabindex="0"></article>
<span tabindex="1"></span>
<!-- static Vue bindings with tabindex on non-interactive elements should also be flagged -->
<div :tabindex="'0'"></div>
<div v-bind:tabindex="'0'"></div>
<div :role="'article'" :tabindex="'0'"></div>
