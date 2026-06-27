<!-- should not generate diagnostics -->

<!--
	For this rule, we treat Svelte shorthand syntax as satisfying all conditions of the rule (both presence and validity of the "type" attribute).
	This is because its non trivial to determine the value of the "type" attribute when using shorthand syntax, so we favor having a false negative over a false positive.
-->
<button {type}>Do something</button>
<button {type} />
<button type={foo} />

<!-- We also should not flag components, only HTML elements -->
<Button>submit</Button>

<!-- Svelte template attribute values with interpolation are treated as dynamic — no false positive -->
<button type="{dynamicType}">Dynamic type</button>
<button type="sub{x}">Mixed type</button>
