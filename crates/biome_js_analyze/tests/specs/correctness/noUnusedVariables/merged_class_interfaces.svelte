<!-- should not generate diagnostics -->
<script module lang="ts">
  class Row { id = 0 }
  interface Row { label: string }
  interface Column { label: string }
  class Column { id = 0 }
</script>
<script lang="ts">
  let { row, column }: { row: Row; column: Column } = $props();
</script>
<p>{row.id} {row.label} {column.id} {column.label}</p>
