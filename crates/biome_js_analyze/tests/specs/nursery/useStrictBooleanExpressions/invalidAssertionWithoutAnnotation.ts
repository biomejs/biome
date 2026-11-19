// should generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
const assertDefined = (value: unknown): asserts value => {
	if (!Boolean(value)) {
		throw new Error();
	}
};

function check(value: string | undefined) {
	assertDefined(value);
	if (value) {}
}
