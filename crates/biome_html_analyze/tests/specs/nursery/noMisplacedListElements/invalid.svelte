<!-- should generate diagnostics -->
<div><li /></div>
<ul>{#if visible}<div><li>Indirect child</li></div>{/if}</ul>
{#snippet item()}
    <div><li>Item in a div</li><li /></div>
{/snippet}
<div>
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
<div><svelte:boundary><li>Boundary item</li></svelte:boundary></div>
<div><slot><li>Slot fallback item</li></slot></div>
<template><div><li>Item in a div</li></div></template>
