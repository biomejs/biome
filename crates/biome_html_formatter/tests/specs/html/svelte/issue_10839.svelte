<script lang="ts"></script>

<div>
	{#each presets as [ preset, label ]}
      <div>{preset}: {label}</div>
    {/each}

	{#each entries as [ key, [ value, description ] ]}
      <div>{key}: {value} ({description})</div>
    {/each}
</div>

<div bind:x={get, set}></div>

<div
	bind:clientWidth={null, (width: number) => {
			if (!vertical) {
				trackSize = width
			}
		}}
	bind:clientHeight={null, (height: number) => {
			if (vertical) {
				trackSize = height
			}
		}}
></div>
