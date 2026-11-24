<!-- should generate diagnostics -->
<script>
  import { writable } from 'svelte/store';
  const counter = writable(0);
  const limit = 10;
  $counter = 1;
</script>
<button onclick={() => { $counter++; limit = 2; }}>Update</button>
<button onclick={(limit) => limit++}>Local</button>
