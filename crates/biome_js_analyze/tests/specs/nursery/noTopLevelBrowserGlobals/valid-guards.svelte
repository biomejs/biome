<!-- should not generate diagnostics -->
<script>
import { browser as client } from "$app/environment";
import * as env from "$app/environment";
import { BROWSER } from "esm-env";
if (client) { window.innerWidth; }
client && document.title;
!client || document.title;
if (!client) {} else { window.innerWidth; }
if (env.browser) { document.title; }
BROWSER ? document.title : "";
if (!import.meta.env.SSR) { document.title; }
import.meta.env.SSR || document.title;
</script>
