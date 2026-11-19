// should generate diagnostics

type Shape =
	| { kind: "circle"; radius: number }
	| { kind: "square"; size: number };

function caseOfDiscriminant(shape: Shape | undefined) {
	switch (shape?.kind) {
		case "circle":
			return shape?.radius;
	}
}

function caseOfLiteral(value: string | null) {
	switch (value) {
		case "a":
			return value?.length;
	}
}

function caseOfNonNullAssertion(value: string | null) {
	switch (value!) {
		case "a":
			return value?.length;
	}
}

function caseOfTypeof(value: string | number | null) {
	switch (typeof value) {
		case "string":
			return value?.length;
	}
}

function caseOfStringThatNamesNoKind(value: { size: number } | null) {
	switch (typeof value) {
		case "strin":
			return value?.size;
	}
}

function switchOnTrue(value: string | null) {
	switch (true) {
		case value !== null:
			return value?.length;
	}
}

function afterExhaustiveSwitch(kind: "a" | "b", label: string | null) {
	switch (kind) {
		case "a":
			if (label === null) return;
			break;
		case "b":
			if (label === null) return;
	}
	return label?.length;
}

function afterExhaustiveSwitchOnTypeParameter<K extends "a" | "b">(
	kind: K,
	label: string | null,
) {
	switch (kind) {
		case "a":
			if (label === null) return;
			break;
		case "b":
			if (label === null) return;
	}
	return label?.length;
}
