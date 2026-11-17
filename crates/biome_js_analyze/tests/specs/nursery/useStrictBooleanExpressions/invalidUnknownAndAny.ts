// should generate diagnostics
// `typeof` with `"object"` and `!== null` narrow `unknown` to `object`, which
// is always truthy, and tests that don't narrow `any` keep `any`.

function objectNotNull(value: unknown) {
	if (typeof value === "object" && value !== null) {
		if (value) {
			return 1;
		}
	}
}

function anyObject(value: any) {
	if (typeof value === "object") {
		if (value) {
			return 1;
		}
	}
}
