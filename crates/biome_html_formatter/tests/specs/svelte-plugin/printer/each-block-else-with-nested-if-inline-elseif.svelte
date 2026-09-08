{#each animals as animal}
    <p>{animal}</p>
{:else}
    {#if type === "dog"}no dogs{:else if type === "cat"}no cats{/if}
{/each}
