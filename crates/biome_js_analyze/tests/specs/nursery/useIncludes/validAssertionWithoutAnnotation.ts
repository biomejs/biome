// should not generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
const assertDefined = (value: unknown): asserts value => {
	if (!value) {
		throw new Error();
	}
};

function hasA(values: string[] | undefined) {
	assertDefined(values);
	return values.indexOf("a") !== -1;
}
