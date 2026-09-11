// should generate diagnostics

// Deleting the guard leaves every case below reported except the last one,
// so they catch narrowing suppressing a diagnostic rather than causing one.
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

interface AsyncFn {
	(): Promise<void>;
}

// The one case that needs narrowing: without the guard, calling
// `number | AsyncFn` resolves to nothing and is not reported.
function guardedCallableInterface(f: number | AsyncFn) {
	if (typeof f === "function") {
		f();
	}
}
