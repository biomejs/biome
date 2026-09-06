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

// A `switch` on the discriminant narrows its cases the same way.
function narrowedToPlainInSwitch(value: Value) {
	switch (value.kind) {
		case "plain":
			value.payload;
			break;
	}
}
