<!-- should generate diagnostics -->
<script>
  let x = 1;
</script>

<!-- Comma operator in template expression - should be flagged -->
<p>{(console.log("side effect"), x)}</p>
