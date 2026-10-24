// should generate diagnostics

function optionalAfterGuard(value: { name: string } | null) {
	if (value == null) return;
	return value?.name;
}

function coalescingAfterTypeof(value: string | undefined) {
	if (typeof value === "string") {
		return value ?? "fallback";
	}
}

function repeatedTruthiness(value: "ready" | null) {
	if (value) {
		if (value) {}
	} else {
		if (value) {}
	}
}

function strictLiteral(value: 0 | 1) {
	if (value === 0) {
		if (value) {}
	}
}

function logicalOperand(value: { name: string } | null) {
	return value !== null && value?.name;
}

function conditionalOperand(value: { name: string } | null) {
	return value !== null ? value?.name : null;
}
