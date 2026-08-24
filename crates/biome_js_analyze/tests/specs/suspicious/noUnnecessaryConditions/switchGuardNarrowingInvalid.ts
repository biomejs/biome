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
