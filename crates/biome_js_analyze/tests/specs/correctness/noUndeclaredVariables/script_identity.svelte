<!-- should generate diagnostics -->
<script module>
  const shared = 1;
  console.log(instanceOnly);
</script>
<script>
  const instanceOnly = shared;
  console.log(content);
</script>
{#snippet content()}<p>{instanceOnly} {missing}</p>{/snippet}
