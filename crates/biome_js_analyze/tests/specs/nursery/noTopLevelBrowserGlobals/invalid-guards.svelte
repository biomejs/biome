<!-- should generate diagnostics -->
<script>
import { browser } from "$app/environment";
if (!browser) { window.innerWidth; }
if (browser) {} else { document.title; }
if (import.meta.env.SSR) { document.title; }
{ const browser = true; if (browser) { window.innerWidth; } }
if (!import.meta.server) { document.title; }
</script>
