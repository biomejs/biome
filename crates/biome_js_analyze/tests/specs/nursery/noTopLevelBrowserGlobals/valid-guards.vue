<!-- should not generate diagnostics -->
<script setup>
import { BROWSER as client } from "esm-env";
if (client) { document.title; }
if (!import.meta.env.SSR) { document.title; }
if (import.meta.env.SSR === false) { document.title; }
if (false === import.meta.env.SSR) { document.title; }
if (import.meta.env.SSR !== true) { document.title; }
if (import.meta.env.SSR === true) {} else { document.title; }
if ((typeof window !== "undefined") === true) { document.title; }
</script>
