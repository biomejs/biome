function suppressedMultiline() {
	return Array.from(things, (thing, index) => (
		// biome-ignore lint/suspicious/noArrayIndexKey: static placeholder
		<div
			key={index}
			className="placeholder"
		>
			{thing}
		</div>
	));
}

function unsuppressedMultiline() {
	return Array.from(things, (thing, index) => (
		<div
			key={index}
			className="placeholder"
		>
			{thing}
		</div>
	));
}
