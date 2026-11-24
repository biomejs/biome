<!-- should not generate diagnostics -->
<script module lang="ts">
  const Label = 'hello';
  type Count = number;
</script>
<script lang="ts">
  type Label = string;
  const Count = 1;
</script>
<p>{Label as Label}: {Count as Count}</p>
