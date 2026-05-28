<script lang="ts">
import type { UnusedType } from "./types";
import type { UsedType } from "./other";
</script>

{#snippet card(p: { value: UsedType })}
  <span>{p.value}</span>
{/snippet}
