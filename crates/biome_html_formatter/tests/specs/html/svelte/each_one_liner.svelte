{#each items as item, i (item.id)} <div>{i}: {item.name}</div> {:else} <div>Empty list</div> {/each}
