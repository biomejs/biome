{#each cells.filter(f => { return f === 1}), item}
	<InputOTP.Slot {cell} />
{/each}

{#each cells.slice(0, 3) as cell, i (i)}
	<InputOTP.Slot {cell} />
{/each}
