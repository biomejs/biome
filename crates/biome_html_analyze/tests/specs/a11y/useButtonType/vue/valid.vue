<!-- should not generate diagnostics -->

<!--
	For this rule, we treat v-binds as if they fully satisfy all conditions of the rule when the value
	cannot be statically determined. Static string literals are validated like plain HTML attributes.
-->
<button v-bind:type="foo">submit</button>
<button :type="foo">submit</button>
<!-- static Vue bindings with valid type values are valid -->
<button :type="'submit'">submit</button>
<button v-bind:type="'button'">click</button>
<button :type="'reset'">reset</button>

<!-- We also should not flag components, only HTML elements -->
<Button>submit</Button>
