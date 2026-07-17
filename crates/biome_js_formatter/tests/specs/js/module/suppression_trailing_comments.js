const config = {
	items:
		// biome-ignore format: preserve the inner array formatting
		[foo  , bar], // keep this trailing comment attached after the suppressed value
	enabled: true,
};

callExpression(
	firstArg,
	// biome-ignore format: preserve the callback body formatting
	function () { return { a: 1,  b: 2 } }, // keep this trailing comment after the suppressed callback argument
	lastArg,
);

const output = wrapper({
	value:
		// biome-ignore format: preserve the conditional expression formatting
		condition?left:right, // keep this trailing comment after the suppressed conditional value
	label: "done",
});
