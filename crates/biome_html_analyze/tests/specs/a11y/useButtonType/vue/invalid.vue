<!-- should generate diagnostics -->

<button>Do something</button>
<button />
<button type="incorrectType">Do something</button>
<button type="incorrectType" />
<button type>Do something</button>
<button type="" />
<!-- static Vue bindings with invalid type values should also be flagged -->
<button :type="'incorrectType'">Do something</button>
<button v-bind:type="'invalid'">Do something</button>
