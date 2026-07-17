<!-- should not generate diagnostics -->
<script>
let items = [];
let rows = [];
let pairs = [];
let matrix = [];
</script>

{#each items as item}
    <span>{item.name}</span>
{/each}

{#each items as item, index}
    <span>{index}: {item.name}</span>
{/each}

{#each items as item (item.id)}
    <span>{item.name}</span>
{/each}

{#each items as item, i (item.id)}
    <span>{i}: {item.name}</span>
{/each}

{#each rows as { id, label }}
    <span>{id}: {label}</span>
{/each}

{#each pairs as [first, second]}
    <span>{first} - {second}</span>
{/each}

{#each rows as { id, ...rest }}
    <span>{id}: {rest}</span>
{/each}

{#each matrix as [[a, b], [c, d]]}
    <span>{a}{b}{c}{d}</span>
{/each}
