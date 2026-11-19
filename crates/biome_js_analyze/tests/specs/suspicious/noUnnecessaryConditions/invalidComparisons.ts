// should generate diagnostics
// Loose equality with a literal, a comparison with a value whose type comes
// from an annotation, and a comparison of a condition with a boolean narrow
// the compared value, so a check that the narrowed value can't fail is
// reported, as in TypeScript.

function looseLiteral(value: "a" | "b" | null) {
	if (value == "a") {
		return value ?? "b";
	}
}

function looseDiscriminant(shape: { kind: "c"; r: number } | { kind: "s"; r: null }) {
	if (shape.kind == "c") {
		return shape.r ?? 0;
	}
}

function annotatedValue(value: string | null, other: string) {
	if (value === other) {
		return value ?? "";
	}
}

function booleanComparison(value: string | null) {
	if ((value !== null) === true) {
		return value ?? "";
	}
}

function aliasNotFalse(value: string | null) {
	const present = value !== null;
	if (present !== false) {
		return value ?? "";
	}
}
