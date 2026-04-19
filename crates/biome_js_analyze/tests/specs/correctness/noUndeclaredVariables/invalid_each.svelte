<!-- should generate diagnostics -->
<script>
let items = [];
let rows = [];
let pairs = [];
</script>

{#each items as item}
    <span>{other.name}</span>
{/each}

{#each items as item, index}
    <span>{i}: {item.name}</span>
{/each}

{#each rows as { id, label }}
    <span>{name}: {label}</span>
{/each}

{#each pairs as [first, second]}
    <span>{third}</span>
{/each}

{#each rows as { id, ...rest }}
    <span>{others}</span>
{/each}

{#each items as item (key)}
    <span>{item.name}</span>
{/each}
