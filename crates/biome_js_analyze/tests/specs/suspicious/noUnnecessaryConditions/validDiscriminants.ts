// should not generate diagnostics
// TypeScript keeps the whole union here: a type-guard call on a property
// doesn't narrow the object, and a property is a discriminant only when some
// member gives it a literal type.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

declare function isLoaded(state: string): state is "loaded";

function guarded(result: Result) {
	if (isLoaded(result.state)) {
		return result.data?.size;
	}
}

function notDiscriminant(
	value: { kind: string; size: number | null } | { kind: string; size: string },
) {
	if (value.kind === "a") {
		return value.size ?? 0;
	}
}
