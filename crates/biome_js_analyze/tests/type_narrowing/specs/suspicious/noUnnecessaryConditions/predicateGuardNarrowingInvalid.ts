// should generate diagnostics

type Box = { a: number };

function isBox(value: unknown): value is Box {
	return typeof value === "object" && value !== null;
}

function alwaysTruthyAfterGuard(value: unknown) {
	if (isBox(value)) {
		// `value` is narrowed to an object here, so this condition is always
		// truthy.
		if (value) {
			value;
		}
	}
}

function isSecondBox(first: unknown, second: unknown): second is Box {
	return typeof second === "object" && second !== null;
}

// The predicate narrows the argument it names, not the first one.
function alwaysTruthySecondArgument(first: unknown, second: unknown) {
	if (isSecondBox(first, second)) {
		if (second) {
			second;
		}
	}
}
