// should generate diagnostics

type Flagged =
	| { kind: "on"; flag: 1 }
	| { kind: "off"; flag: 0 };

function alwaysTruthyAfterGuard(x: Flagged) {
	if (x.kind === "on") {
		// `x.flag` is narrowed to `1` here, so this condition is always
		// truthy.
		if (x.flag) {
			x;
		}
	}
}
