<!-- should generate diagnostics -->
<script>
    const rows = [1];
    const item = 0;
    const current = 0;
    const param = 0;
</script>
{#each rows as item (item)}
    {#snippet draw(param)}{param}{/snippet}
    {@render draw(item)}
    {@const sibling = item}
{:else}
    {const sibling = 1}
    {sibling}
{/each}
{#await rows then current}{current}{/await}
