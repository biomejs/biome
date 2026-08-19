<!-- should not generate diagnostics -->
<script lang="ts">
import { inView } from "./actions";
import { fade } from "./transitions";
import { cdnUrl } from "./config";
import type { IconType } from "./icons";
</script>

<div use:inView transition:fade style="background: url({cdnUrl})"></div>

{#snippet card(p: { icon: IconType })}
  <span>{p.icon}</span>
{/snippet}
