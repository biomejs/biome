// should generate diagnostics
// TypeScript ignores assertion functions declared as variables without a type
// annotation (TS2775), so `value` keeps its declared type after the call.
const assertNumber = (value: unknown): asserts value is number => {
	if (typeof value !== "number") {
		throw new Error();
	}
};

function increment(value: number | bigint) {
	assertNumber(value);
	return value + 1;
}
