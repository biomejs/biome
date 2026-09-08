{#each animals as animal}
    <p>{animal}</p>
{:else}
    {#if type === "dog"}
        <p>no dogs</p>
    {:else}
        <p>no animals</p>
    {/if}
{/each}
