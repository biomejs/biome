{#each ids as id}
    {#each objs as { [id]: value }}
        <td>{value}</td>
    {/each}
{/each}
