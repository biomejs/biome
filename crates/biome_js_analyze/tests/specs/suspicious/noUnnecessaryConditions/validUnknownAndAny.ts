// should not generate diagnostics
// `!== null` leaves `{} | undefined`, a `switch` keeps `unknown` where no case
// matches, and tests that don't narrow `any` keep `any`, so these checks can
// fail.

function strictNull(value: unknown) {
	if (value !== null) {
		return value ?? 1;
	}
}

function switchDefault(value: unknown) {
	switch (typeof value) {
		case "undefined":
		case "object":
			return 0;
		default:
			return value ?? 1;
	}
}

function anyTruthy(value: any) {
	if (value) {
		return value ?? 1;
	}
}

function anyObject(value: any) {
	if (typeof value === "object") {
		return value?.data;
	}
}

function anyElse(value: any) {
	if (value instanceof Error) {
		return 0;
	}
	return value?.message;
}

function parsedElse(text: string) {
	const value = JSON.parse(text);
	if (value instanceof Error) {
		return 0;
	}
	return value?.message;
}
