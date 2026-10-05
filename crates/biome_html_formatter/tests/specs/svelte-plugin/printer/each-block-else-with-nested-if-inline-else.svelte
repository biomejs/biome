{#each animals as animal}
    <p>{animal}</p>
{:else}
    {#if type === "dog"}no dogs{:else}no animals{/if}
{/each}
