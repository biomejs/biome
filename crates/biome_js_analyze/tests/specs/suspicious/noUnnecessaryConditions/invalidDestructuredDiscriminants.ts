// should generate diagnostics
// A test of one element of a destructuring pattern narrows the value that the
// pattern destructures, so another element of the same pattern takes its type
// from the narrowed members, as in TypeScript.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

function parameter({ state, data }: Result) {
	if (state === "loaded") {
		return data?.size;
	}
}

function declared(result: Result) {
	const { state, data } = result;
	if (state !== "loaded") {
		if (data) {
			return 1;
		}
	}
}

function aliased({ state, data }: Result) {
	const loaded = state === "loaded";
	if (loaded) {
		return data?.size;
	}
}

function rest(...[state, data]: ["loaded", { size: number }] | ["failed", null]) {
	if (state === "loaded") {
		return data?.size;
	}
}

function asserted(result: Result | undefined) {
	const { state, data } = result!;
	if (state === "loaded") {
		return data?.size;
	}
}
