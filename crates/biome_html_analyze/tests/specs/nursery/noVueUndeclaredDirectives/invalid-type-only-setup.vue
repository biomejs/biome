<!-- should generate diagnostics -->
<script setup lang="ts">
type vTypeAlias = {};
interface vInterface {}
import type { vImportedType } from "./types";
import { type vInlineType } from "./types";
import type vDefaultType from "./types";
</script>

<template>
    <div v-type-alias></div>
    <div v-interface></div>
    <div v-imported-type></div>
    <div v-inline-type></div>
    <div v-default-type></div>
</template>
