<!-- should generate diagnostics -->
<script module>
  const shared = 1;
</script>
<script>
  const instance = shared;
  const componentOnly = 2;
</script>
<div>
  <script type="module">
    const nestedOnly = 3;
    const local = 4;
    console.log(local, componentOnly);
  </script>
</div>
<p>{instance} {nestedOnly}</p>
