<script lang="ts"></script>

<button
    onclick={(e: Event) => {
        e as any;
    }}>foo</button
>

<button onclick={(e: Event) => e.detail}>{bar as any}</button>
<input bind:value={(config as Config).value} />
<input bind:value={config[x].y} />

{#await promise}
    <p>Waiting</p>
{:then num: number}
    <p>{num}</p>
{:catch error: Error}
    <p>Error: {error.message}</p>
{/await}

{#each [] as x: number}
    {x}
{/each}
