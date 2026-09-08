{#each animals as animal}
    <p>{animal}</p>
{:else}
    {#if type === "dog"}
        <p>no dogs</p>
    {/if}
{/each}
