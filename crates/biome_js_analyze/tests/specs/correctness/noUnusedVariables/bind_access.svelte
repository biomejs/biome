<!-- should generate diagnostics -->
<script>
  let element;
  let value = $state('');
  const refs = {};
</script>

<input bind:this={element} bind:value />
<button bind:this={refs.button}>Submit</button>
