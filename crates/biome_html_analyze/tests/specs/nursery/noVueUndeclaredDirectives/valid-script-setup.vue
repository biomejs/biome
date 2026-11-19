<!-- should not generate diagnostics -->
<script setup>
import vImportedDirective from "./directive";
import { importedDirective as vAliasedDirective } from "./directive";

const vLocalDirective = {};
const { vDestructuredDirective } = { vDestructuredDirective: {} };
</script>

<template>
    <div v-imported-directive></div>
    <div v-aliased-directive></div>
    <div v-local-directive></div>
    <div v-destructured-directive></div>
</template>
