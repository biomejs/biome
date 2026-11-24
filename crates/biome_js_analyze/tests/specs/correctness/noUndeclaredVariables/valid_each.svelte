<!-- should not generate diagnostics -->
<script lang="ts">
let items = [];
let rows = [];
let pairs = [];
let matrix = [];
let fallback = 0;
type Row = { id: number };
type Pair<A, B> = [A, B];
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

{#each rows as { id = fallback, label: title = "untitled" }}
    <span>{id}: {title}</span>
{/each}

{#each pairs as [first = fallback, second = [fallback]]}
    <span>{first}: {second}</span>
{/each}

{#each items as key}
    {#each rows as { [key]: value }}
        <span>{value}</span>
    {/each}
{/each}

{#each rows as { list: [first, ...rest] }}
    <span>{first}: {rest}</span>
{/each}

{#each rows as { [items[0]?.id]: [computedFirst, computedSecond] }}
    <span>{computedFirst}: {computedSecond}</span>
{/each}

{#each pairs as [head, ...{ length }]}
    <span>{head}: {length}</span>
{/each}

{#each rows as row: Row}
    <span>{row.id}</span>
{/each}

{#each pairs as pair: Pair<number, string>, index (pair)}
    <span>{index}: {pair}</span>
{/each}
