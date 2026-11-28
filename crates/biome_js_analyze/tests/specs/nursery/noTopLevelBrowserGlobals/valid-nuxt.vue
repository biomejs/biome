<!-- should not generate diagnostics -->
<script setup>
if (import.meta.client) { document.title; }
if (!import.meta.server) { document.title; }
if (import.meta.server) {} else { document.title; }
import.meta.client && window.innerWidth;
</script>
