// should generate diagnostics

function alwaysTruthyCase(x: "on" | "") {
	switch (x) {
		case "on":
			// `x` is narrowed to `"on"` here, so this condition is always
			// truthy.
			if (x) {
				x;
			}
			break;
	}
}

// The empty-string case narrows `x` to `""`, so the condition is always falsy.
function alwaysFalsyCase(x: "on" | "") {
	switch (x) {
		case "":
			if (x) {
				x;
			}
			break;
	}
}
