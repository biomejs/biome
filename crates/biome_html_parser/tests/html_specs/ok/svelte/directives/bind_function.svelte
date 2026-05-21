<input bind:value={get, set} />
<input bind:value={null, setter} />
<Toggle bind:pressed={getToggled, setToggled} />
<input bind:value={() => [a, b], (v) => set(v)} />
<input bind:value={
	() => value,
	(v) => value = v.toLowerCase()
} />
