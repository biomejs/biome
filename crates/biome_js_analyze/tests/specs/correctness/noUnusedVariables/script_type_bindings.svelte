<!-- should generate diagnostics -->
<script module lang="ts">
  interface Row { id: number }
  interface Row { label: string }
  const Row = 1;
  type T = boolean;
  const T = 2;
  type Count = number;
</script>
<script lang="ts" generics="T">
  let { value, row, count }: { value: T; row: Row; count: Count } = $props();
  console.log(value, row, count, T);
</script>
