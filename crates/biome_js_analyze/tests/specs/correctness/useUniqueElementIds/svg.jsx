// SVG elements with static id attributes should not be flagged
function SvgWithIds() {
	return (
		<svg>
			<defs>
				<linearGradient id="gradient1">
					<stop offset="0%" stopColor="red" />
					<stop offset="100%" stopColor="blue" />
				</linearGradient>
				<pattern id="pattern1" width="10" height="10">
					<circle cx="5" cy="5" r="3" fill="blue" />
				</pattern>
				<clipPath id="clip1">
					<rect x="0" y="0" width="100" height="100" />
				</clipPath>
			</defs>
			<rect id="rect1" width="100" height="100" fill="url(#gradient1)" />
			<circle id="circle1" cx="50" cy="50" r="25" />
		</svg>
	);
}

// Nested SVG elements should also not be flagged
function NestedSvg() {
	return (
		<div>
			<svg>
				<g id="group1">
					<rect id="rect2" width="50" height="50" />
					<svg>
						<circle id="circle2" cx="25" cy="25" r="10" />
					</svg>
				</g>
			</svg>
		</div>
	);
}

// HTML elements outside SVG should still be flagged (these should trigger diagnostics)
function HtmlWithIds() {
	return (
		<div>
			<div id="shouldBeFlagged1">Content</div>
			<svg>
				<rect id="shouldNotBeFlagged" width="50" height="50" />
			</svg>
			<p id="shouldBeFlagged2">More content</p>
		</div>
	);
}

// Mixed case: SVG elements should not be flagged, but HTML elements should
function MixedContent() {
	return (
		<div id="htmlDiv">
			<svg>
				<defs>
					<linearGradient id="svgGradient">
						<stop offset="0%" />
					</linearGradient>
				</defs>
				<rect id="svgRect" fill="url(#svgGradient)" />
			</svg>
			<span id="htmlSpan">Text</span>
		</div>
	);
}