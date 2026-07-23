<!-- should generate diagnostics -->
<div scope="col"></div>
<div v-bind:scope="scopeValue"></div>
<div :scope="scopeValue"></div>
<div scope="col"></div>
<div scope></div>
<td scope="col"></td>
<span scope="row"></span>
<p scope></p>
<!-- static Vue bindings on non-<th> elements should also be flagged -->
<div :scope="'col'"></div>
<td v-bind:scope="'row'"></td>
