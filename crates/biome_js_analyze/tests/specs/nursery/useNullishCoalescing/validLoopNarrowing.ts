// should not generate diagnostics

function afterContinue(values: (string | null)[]) {
	for (const value of values) {
		if (value === null) continue;
		const label = value || "none";
	}
}

function forInObject(record: { a: number } | undefined) {
	for (const key in record) {
		const a = record || { a: 0 };
	}
}
