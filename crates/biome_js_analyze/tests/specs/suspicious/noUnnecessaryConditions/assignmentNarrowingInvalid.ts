// should generate diagnostics

function alwaysTruthyAfterAssignment(x: string | undefined) {
	x = "on";
	// `x` is narrowed to `"on"` here, so this condition is always truthy.
	if (x) {
		x;
	}
}

function alwaysFalsyAfterAssignment(x: string | undefined) {
	x = undefined;
	// `x` is narrowed to `undefined` here, so this condition is always falsy.
	if (x) {
		x;
	}
}
