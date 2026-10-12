<!-- should generate diagnostics -->
<script>
  import { toasts } from "./stores.js";
  import { theme } from "./stores.js";
  import { unused } from "./stores.js";
</script>

{#each $toasts as toast (toast.id)}
  <p>{toast.title}</p>
{/each}

<select bind:value={$theme}>
  <option value="white">White</option>
</select>
