// should generate diagnostics

function narrowedToPromise(x: number | Promise<void>) {
	if (typeof x === "object") {
		x;
	}
}

function guardedCall(x: number | (() => Promise<void>)) {
	if (typeof x === "function") {
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
