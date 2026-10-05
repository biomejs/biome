{#each boxes as box}
    {const area = box.width * box.height}
    {const label = (value) => `${value} square pixels`}
    <p>{label(area)}</p>
{/each}

{#if user}
    {let name = $state(user.name)}
    {let greeting = $derived(`Hello ${name}`)}
    <input bind:value={name} />
    <p>{greeting}</p>
{/if}

<Component>
    {let value = getValue()}
</Component>
