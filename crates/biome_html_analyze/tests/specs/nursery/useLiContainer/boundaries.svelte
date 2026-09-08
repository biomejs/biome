<!-- should generate diagnostics -->
<li>Root item</li>
<li />
{#if visible}<li>Root conditional item</li>{/if}
<List><li>Component child</li><li /></List>
<svelte:element this="ul"><li>Dynamic element child</li><li /></svelte:element>
<List><svelte:fragment slot="items"><li>Slotted item</li><li /></svelte:fragment></List>
{#snippet item()}
    <li>Snippet item</li>
    <li />
{/snippet}
<div>
    {#snippet nestedItem()}
        <li>Nested snippet item</li>
        <li />
        {#if visible}<li>Conditional snippet item</li>{/if}
    {/snippet}
    {#if visible}
        <li>Conditional item</li>
    {:else if pending}
        <li>Pending item</li>
    {:else}
        <li />
    {/if}
    {#each items as item}
        <li>{item}</li>
    {/each}
    {#await promise}
        <li>Loading</li>
    {:then item}
        <li>{item}</li>
    {:catch error}
        <li>{error}</li>
    {/await}
    {#key value}
        <li />
    {/key}
</div>
<ul>
    {#if visible}
        {#each items as item}
            <li>{item}</li>
        {/each}
    {/if}
</ul>
