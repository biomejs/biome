// should generate diagnostics

function nullishGuard(value: Promise<void> | null) {
	if (value !== null) {
		value;
	}
}

function typeofGuard(value: Promise<void> | string) {
	if (typeof value === "string") return;
	value;
}

function truthinessGuard(value: Promise<void> | false) {
	if (value) {
		value;
	}
}
