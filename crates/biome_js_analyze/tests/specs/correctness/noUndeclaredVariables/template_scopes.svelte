<!-- should generate diagnostics -->
<script lang="ts">
    const rows = [{ id: 1 }];
    const promise = Promise.resolve(rows[0]);
</script>
{#each rows as { id: item }, i (item + i)}
    {const label = item}
    {#snippet card<T>(value: T)}{value as T} {label}{/snippet}
    {@render card(label)}
{:else}
    {item} {i} {label} {@render card(1)}
{/each}
{value}
{#await promise}
    {result}
{:then { id: result }}
    {result} {error}
{:catch error}
    {error} {result}
{/await}
{#await promise then current}{current.id}{/await}
{#await promise catch problem}{problem}{/await}
{current} {problem}
