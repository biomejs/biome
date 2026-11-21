// should not generate diagnostics
// A `const` holds a discriminant only when it has no type annotation and no
// default, so these tests leave the whole union, as in TypeScript. In
// `written` and `writtenCondition`, TypeScript narrows `result` to `Loaded`,
// but the write can change the state after the `const` reads it, so
// `result.data` can be `null`.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

function annotated(result: Result) {
	const state: string = result.state;
	if (state === "loaded") {
		return result.data?.size;
	}
}

function mutable(result: Result) {
	let state = result.state;
	if (state === "loaded") {
		return result.data?.size;
	}
}

function defaulted(result: Result) {
	const { state = "loaded" } = result;
	if (state === "loaded") {
		return result.data?.size;
	}
}

function written(result: Result) {
	const state = result.state;
	result.state = "failed";
	if (state === "loaded") {
		return result.data?.size;
	}
}

function writtenCondition(result: Result) {
	const loaded = result.state === "loaded";
	result.state = "failed";
	if (loaded) {
		return result.data?.size;
	}
}
