function narrowedToPromise(x: number | Promise<void>) {
	if (typeof x === "object") {
		// `x` is narrowed to `Promise<void>` here, so the bare statement floats.
		x;
	}
}

function guardedCall(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
		// `x` is narrowed to `() => Promise<void>` here, so the call floats.
		x();
	}
}

function reversedOperands(x: number | (() => Promise<void>)) {
	if ("function" === typeof x) {
		x();
	}
}

function looseEquality(x: number | (() => Promise<void>)) {
	if (typeof x == "function") {
		x();
	}
}
