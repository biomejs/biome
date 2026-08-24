// should generate diagnostics

type Value =
	| { kind: "promise"; payload: Promise<void> }
	| { kind: "plain"; payload: number };

function guardedCase(value: Value) {
	switch (value.kind) {
		case "promise":
			// `value` is narrowed to the promise variant here, so the bare
			// member statement floats.
			value.payload;
			break;
	}
}

function groupedCases(value: Value) {
	switch (value.kind) {
		case "promise":
		case "plain":
			// Reachable with both variants; narrowing must not apply, so the
			// promise variant keeps the diagnostic.
			value.payload;
			break;
	}
}

function fallthroughCase(value: Value) {
	switch (value.kind) {
		case "promise":
			console.log("promise");
		case "plain":
			// Reachable by falling through from the promise case; narrowing
			// must not apply.
			value.payload;
			break;
	}
}
