<!-- should not generate diagnostics -->
<th scope="col"></th>
<th scope="row"></th>
<th v-bind:scope="scopeValue"></th>
<th :scope="scopeValue"></th>
<th scope="col"></th>
<th scope></th>
<div></div>
<td></td>
<!-- static Vue bindings on <th> are valid -->
<th :scope="'col'"></th>
<th v-bind:scope="'row'"></th>
