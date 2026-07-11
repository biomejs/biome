<!-- should not generate diagnostics -->
<script lang="ts">
import type { IconType } from "./icons";
import { renderIcon } from "./render";
</script>

{#snippet card(p: { icon: IconType })}
  <span>{renderIcon(p.icon)}</span>
{/snippet}
