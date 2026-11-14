// should not generate diagnostics
// TypeScript keeps the declared types of these elements: the pattern is
// declared with `let`, a variable of the parameter is written, the read
// element has a default, the pattern has only one element, or the tested
// element is not a discriminant.

interface Loaded {
	state: "loaded";
	data: { size: number };
}

interface Failed {
	state: "failed";
	data: null;
}

type Result = Loaded | Failed;

function mutable(result: Result) {
	let { state, data } = result;
	if (state === "loaded") {
		return data?.size;
	}
}

function written({ state, data }: Result, fallback: { size: number }) {
	if (state === "failed") {
		data = fallback;
	}
	if (state === "loaded") {
		return data?.size;
	}
}

function withDefault(result: Result) {
	const { state, data = null } = result;
	if (state === "loaded") {
		return data?.size;
	}
}

function separate(result: Result) {
	const { state } = result;
	const { data } = result;
	if (state === "loaded") {
		return data?.size;
	}
}

function notDiscriminant(
	value: { kind: string; size: number | null } | { kind: string; size: string },
) {
	const { kind, size } = value;
	if (kind === "a") {
		return size ?? 0;
	}
}
