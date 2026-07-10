<script lang="ts">
import { IconType } from "./icons";
</script>

{#snippet card(p: { icon: IconType })}
  <span>{p.icon}</span>
{/snippet}
