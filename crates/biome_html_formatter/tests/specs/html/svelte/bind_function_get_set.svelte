<!-- short: stays on one line -->
<div bind:x={get, set}></div>
<div bind:value={null, setValue}></div>

<!-- medium: still fits when line width allows -->
<div bind:clientWidth={null, (w) => { x = w }}></div>

<!-- forced break with long names / body -->
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
