<!-- should not generate diagnostics -->
<script>
  let count = 0;
  let total = 0;
</script>

<button
  on:click={() => {
    console.log(`id-${count++}`);
  }}
>
  Log ID
</button>

<button on:click={() => (total += 1)}> Increment </button>
