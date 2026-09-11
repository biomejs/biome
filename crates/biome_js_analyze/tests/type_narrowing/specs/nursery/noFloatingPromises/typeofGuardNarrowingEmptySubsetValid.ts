/* should not generate diagnostics */

// Each guard contradicts the declared type, so `x` is `never` inside the
// `if`, and neither `never` nor the result of calling it is a promise.
function impossibleGuard(x: Promise<void>) {
	if (typeof x === "number") {
		x;
	}
}

function impossibleGuardMultiMemberUnion(x: string | Promise<void>) {
	if (typeof x === "boolean") {
		x;
	}
}

function impossibleGuardOnCallable(x: (() => Promise<void>) | (() => void)) {
	if (typeof x === "object") {
		x();
	}
}
