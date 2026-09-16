{#each items as item}
    <b>{item.name}</b>{#if item.isNew}*{/if}{#if item.isUpdated}!{/if}
{/each}
