/* should not generate diagnostics */

// Narrowing `x` (declared as `Promise<void>`) by a `typeof` tag it can never
// have eliminates every union member. The branch is statically unreachable,
// so the narrowed type is `never`, not `Promise<void>` -- there is nothing
// here that could be a floating promise.
function impossibleGuard(x: Promise<void>) {
	if (typeof x === "number") {
		x;
	}
}
