// should generate diagnostics

function alwaysTruthyAfterGuard(x: number | (() => void)) {
	if (typeof x === "function") {
		if (x) {
			x();
		}
	}
}
