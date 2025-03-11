// https://github.com/rome/tools/issues/3735

function supported1(){
	return (
		// biome-ignore format: Work around https://github.com/rome/tools/issues/3734
		// biome-ignore lint/style/useOptionalChain: Optional chaining creates more complicated ES2019 code
		a && b
	);
}

function supported2(){
	return !(
		// biome-ignore format: Work around https://github.com/rome/tools/issues/3734
		// biome-ignore lint/style/useOptionalChain: Optional chaining creates more complicated ES2019 code
		a && b
	);
}

function supported3(){
	return (
		// biome-ignore format:
		aVeryLongLogicalExpression &&
		thatBreaksOverMultipleLines
	);
}
