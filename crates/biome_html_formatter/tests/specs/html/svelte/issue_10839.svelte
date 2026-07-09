<div>
	{#each presets as [ preset, label ]}
      <div>preset</div>
    {/each}
</div>

<div
	bind:clientWidth={null, (width) => {
if (!vertical) {
trackSize = width
}
}}
	bind:clientHeight={null, (height) => {
if (vertical) {
trackSize = height
}
}}
></div>
