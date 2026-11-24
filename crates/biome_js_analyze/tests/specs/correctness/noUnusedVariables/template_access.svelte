<!-- should generate diagnostics -->
<script>
  import * as UI from './ui';
  import { writable } from 'svelte/store';
  const counter = writable(0);
  const label = 'Increment';
  let discarded;
  let total = $state(0);
</script>

<UI.Button {label} onclick={() => { discarded = 1; total += 1; $counter = 2; }} />
