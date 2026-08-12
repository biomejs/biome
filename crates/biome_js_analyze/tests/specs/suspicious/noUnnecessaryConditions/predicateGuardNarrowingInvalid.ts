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
