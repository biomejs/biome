<!-- should not generate diagnostics -->
<div v-if="ok"></div>
<div v-if="a < b"></div>
<div v-if="a"></div>
<div v-else-if="b"></div>
<div v-else></div>
<div id="x" class="y" v-if="flag"></div>

<!-- Nested conditionals with self-closing tag -->
<div v-if="cond1"></div>
<div v-else><span v-if="cond2"/></div>