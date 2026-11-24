<!-- should generate diagnostics -->
<script module lang="ts">
  import { Entry } from './types';
  export type Entries = Entry[];
</script>
<script lang="ts">
  import Row from './Row.svelte';
  const Entry = 'row';
</script>

<Row label={Entry} />
