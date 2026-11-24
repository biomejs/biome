<!-- should not generate diagnostics because the external instance script may write hello -->
<script module>
let hello = "Hello World";
</script>
<script src="./external.js"></script>

<span>{hello}</span>
