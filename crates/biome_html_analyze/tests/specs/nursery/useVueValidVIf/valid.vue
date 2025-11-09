<!-- should not generate diagnostics -->
<div v-if="ok"></div>
<div v-if="a < b"></div>
<div v-if="a"></div>
<div v-else-if="b"></div>
<div v-else></div>
<div id="x" class="y" v-if="flag"></div>