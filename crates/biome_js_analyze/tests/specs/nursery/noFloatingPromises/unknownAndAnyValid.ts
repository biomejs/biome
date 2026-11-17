// should not generate diagnostics
// The `else` branch of `instanceof` and tests that don't narrow `any` keep
// `any`.

function anyElse(value: any) {
	if (value instanceof Promise) {
		return;
	}
	value;
}

function anyObject(value: any) {
	if (typeof value === "object") {
		value;
	}
}
