{#if foo}
    {#if bar}
        x
    {/if}
{:else}
    {#if bar}
        x
    {/if}
{/if}

{#if foo}{#if bar}x{/if}{:else}{#if bar}x{/if}{/if}
