// https://github.com/rome/tools/issues/3735

function supported1(){
	return (
		// rome-ignore format: Work around https://github.com/rome/tools/issues/3734
		// rome-ignore lint/style/useOptionalChain: Optional chaining creates more complicated ES2019 code
		a && b
	);
}

function supported2(){
	return !(
		// rome-ignore format: Work around https://github.com/rome/tools/issues/3734
		// rome-ignore lint/style/useOptionalChain: Optional chaining creates more complicated ES2019 code
		a && b
	);
}

function supported3(){
	return (
		// rome-ignore format:
		aVeryLongLogicalExpression &&
		thatBreaksOverMultipleLines
	);
}
