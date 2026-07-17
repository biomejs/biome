<!-- should generate diagnostics -->
<script>
let items = [];
let total = 0;
</script>

{#each items as item}
    {@const price = unknownPrice}
    <span>{price}</span>
{/each}

{@debug total, missingVar}
{@debug anotherMissing}
