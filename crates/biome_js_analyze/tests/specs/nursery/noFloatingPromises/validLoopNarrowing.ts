// should not generate diagnostics

function nullInForOf(values: (Promise<void> | null)[]) {
	for (const value of values) {
		if (value === null) {
			value;
		}
	}
}

function stringAfterContinue(values: (Promise<void> | string)[]) {
	for (const value of values) {
		if (typeof value !== "string") continue;
		value;
	}
}
