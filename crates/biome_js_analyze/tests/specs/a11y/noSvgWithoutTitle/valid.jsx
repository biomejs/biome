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

	<svg aria-hidden="true">
        <defs>
          <pattern>
            <path d="M.5 200V.5H200" fill="none" />
          </pattern>
        </defs>
    </svg>
</>;
