<!-- should not generate diagnostics -->
<!-- `src` moves the whole script block, including any `directives` option, into a file Biome does not read -->
<script src="./options.js" />

<template>
    <div v-may-come-from-the-external-script></div>
</template>
