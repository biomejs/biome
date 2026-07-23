<!-- 1 -->
{#each items as item, i (item.id)}
	<!-- 2 -->
	<div>{i}: {item.name}</div>
	<!-- 3 -->
{:else}
	<!-- 4 -->
	<div>Empty list</div>
	<!-- 5 -->
{/each}
<!-- 6 -->
