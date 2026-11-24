<!-- should not generate diagnostics -->
{#each [1] as value}
    {let mutable = value}
    <button onclick={() => mutable++}>Update</button>
{/each}
