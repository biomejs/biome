// should not generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
const assertString = (value: unknown): asserts value is string => {
	if (typeof value !== "string") {
		throw new Error();
	}
};

function discard(value: string | undefined) {
	assertString(value);
	return void value;
}
