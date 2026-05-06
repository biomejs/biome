<!-- should not generate diagnostics -->

<!-- img with dynamic :alt binding (Vue shorthand) -->
<img src="image.png" :alt="description" />

<!-- img with dynamic v-bind:alt binding (Vue explicit) -->
<img src="image.png" v-bind:alt="description" />

<!-- img with dynamic :alt and :src -->
<img :src="image.src" :alt="image.description" />
