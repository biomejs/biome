<!-- should not generate diagnostics -->
<div v-once></div>

<!-- ensure other directives do not trigger this rule -->
<div v-text="foo"></div>
