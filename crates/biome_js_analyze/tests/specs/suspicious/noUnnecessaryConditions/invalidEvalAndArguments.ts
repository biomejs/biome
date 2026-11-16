// should generate diagnostics
// TypeScript ignores `eval` and the `arguments` object when narrowing, so
// `value` is a `string` at each optional chain.

function directEval(value: string | null) {
	if (value !== null) {
		eval("value = null");
		return value?.length;
	}
}

function escapedEval(value: string | null) {
	if (value !== null) {
		\u0065val("value = null");
		return value?.length;
	}
}

function mappedArguments(value: string | null) {
	if (value !== null) {
		arguments[0] = null;
		return value?.length;
	}
}
