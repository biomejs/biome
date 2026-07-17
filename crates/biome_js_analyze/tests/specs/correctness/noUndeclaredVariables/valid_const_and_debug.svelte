<!-- should not generate diagnostics -->
<script>
let items = [];
let total = 0;
let label = "hello";
</script>

{#each items as item}
    {@const price = item.price}
    {@const tax = price * 0.2}
    <span>{price} + {tax}</span>
{/each}

{@debug total}
{@debug total, label}
{@debug items, total, label}
