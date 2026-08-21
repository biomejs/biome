/* should not generate diagnostics */

type Value =
	| { kind: "promise"; payload: Promise<void> }
	| { kind: "plain"; payload: number };

// `value` is narrowed to the plain variant, so its payload cannot be a
// promise.
function narrowedToPlain(value: Value) {
	if (value.kind === "plain") {
		value.payload;
	}
}
