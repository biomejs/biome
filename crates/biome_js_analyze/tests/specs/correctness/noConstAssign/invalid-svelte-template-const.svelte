<!-- should generate diagnostics -->
{#each [1] as value}
    {@const legacy = value}
    {const declared = value}
    <button onclick={() => { legacy++; declared++; }}>Update</button>
{/each}
