// should generate diagnostics
// A test of a `const` that holds a discriminant narrows the object that the
// `const` reads it from, so the narrowed members decide the type of the
// object's other properties, as in TypeScript.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

function held(result: Result) {
	const state = result.state;
	if (state === "loaded") {
		return result.data?.size;
	}
}

function destructured(result: Result) {
	const { state } = result;
	if (state !== "loaded") {
		if (result.data) {
			return 1;
		}
	}
}

function aliased(result: Result) {
	const { state: current } = result;
	const loaded = current === "loaded";
	if (loaded) {
		return result.data?.size;
	}
}
