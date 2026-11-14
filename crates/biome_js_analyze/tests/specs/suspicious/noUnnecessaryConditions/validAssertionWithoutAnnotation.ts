// should not generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
const assertDefined = (value: unknown): asserts value => {
	if (!value) {
		throw new Error();
	}
};

const assertDefinedExpression = function (value: unknown): asserts value {
	if (!value) {
		throw new Error();
	}
};

function arrow(value: string | null) {
	assertDefined(value);
	if (value === null) {}
}

function functionExpression(value: string | null) {
	assertDefinedExpression(value);
	return value ?? "default";
}
