{#each " as " as character}
	{character}
{/each}

{#each ` as ` as character}
	{character}
{/each}

{#each / as /.exec("value") ?? [] as match}
	{match}
{/each}

{#each values /* as */ as value}
	{value}
{/each}

{#each values // as
 as value}
	{value}
{/each}

{#each values . as as value}
	{value}
{/each}

{#await " then " then value}
	{value}
{/await}

{#await value /* then */ then result}
	{result}
{/await}

{#await promise . then(resolve) then result}
	{result}
{/await}

{#await promise ?. then(resolve) then result}
	{result}
{/await}

{#await condition ? then : fallback then result}
	{result}
{/await}

{#await /then/ then result}
	{result}
{/await}

{#await "6" / 2 then result}
	{result}
{/await}

{#await `6` / 2 then result}
	{result}
{/await}

{#await /6/ / 2 then result}
	{result}
{/await}

<p>{"6" / 2}</p>
<p>{`6` / 2}</p>

{#await " catch " catch error}
	{error}
{/await}

{#await value /* catch */ catch error}
	{error}
{/await}

{#await promise . catch(handle) catch error}
	{error}
{/await}

{#await left < right then result}
	{result}
{/await}

{#each left < right ? values : fallback as value}
	{value}
{/each}

{#each values as value (value.rank < limit ? value.rank : limit)}
	{value}
{/each}
