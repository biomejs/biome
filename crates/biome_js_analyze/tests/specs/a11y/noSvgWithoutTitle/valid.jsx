/* should not generate diagnostics */

<>
	<svg>
		<title>Pass</title>
		<circle />
	</svg>
	<svg>
		<rect />
		<rect />
		<g>
			<circle />
			<circle />
			<g>
				<title>Pass</title>
				<circle />
				<circle />
			</g>
		</g>
	</svg>
	<svg role="img" aria-label="title">
		<title id="title">Pass</title>
	</svg>
	<svg role="img" aria-label="title">
		<span>Pass</span>
	</svg>
	<svg role="img" aria-label="title">
		<span id="sample">Pass</span>
	</svg>
	<svg role="img" aria-labelledby="title">
		<title id="title">Pass</title>
	</svg>
	<svg role="img" aria-labelledby="title">
		<span id="title">Pass</span>
	</svg>
	<svg role="">
		<title>implicit role</title>
		<span>Pass</span>
	</svg>
	<svg role="presentation">
		<title>presentation role with title</title>
		<span id="sample">Pass</span>
	</svg>
	<svg role="button">
		<title>button role with title</title>
		<span id="sample">Pass</span>
	</svg>
</>;
