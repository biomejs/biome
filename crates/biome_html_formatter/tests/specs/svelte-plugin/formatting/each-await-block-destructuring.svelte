{#each arr as {  a,b =''}}
    {a}
    {b}
{/each}

{#await promise then {  a,b =''}}
    {a}
    {b}
{/await}

{#await promise}
    Loading
{:then {  a,b =''}}
    {a}
    {b}
{/await}
