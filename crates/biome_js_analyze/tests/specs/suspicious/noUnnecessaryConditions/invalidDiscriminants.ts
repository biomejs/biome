// should generate diagnostics
// A test of a discriminant narrows the object that holds it, so the narrowed
// members decide the type of its other properties, as in TypeScript.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

function size(result: Result) {
	if (result.state === "loaded") {
		return result.data?.size;
	}
}

function failed(result: Result) {
	if (result.state !== "loaded") {
		if (result.data) {
			return 1;
		}
	}
}

function aliased(result: Result) {
	const loaded = result.state === "loaded";
	if (loaded) {
		return result.data?.size;
	}
}
