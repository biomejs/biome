<!-- should generate diagnostics -->
<script setup>
if (import.meta.client) { document.title; }
if (!import.meta.server) { document.title; }
if (import.meta.env.SSR == false) { document.title; }
if (import.meta.env.SSR === true) { document.title; }
if (import.meta.env.SSR !== false) { document.title; }
if ((typeof window !== "undefined" && ready) !== false) { document.title; }
</script>
