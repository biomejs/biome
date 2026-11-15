// should generate diagnostics
// Conditions in the body of a `for...of` or `for...in` loop narrow as
// anywhere else, and the body of `for (const key in record)` sees `record` as
// not nullish, as in TypeScript.

function afterContinue(values: (string | null)[]) {
	for (const value of values) {
		if (value === null) continue;
		value?.length;
	}
}

function destructuredElement(entries: [string, { size: number } | null][]) {
	for (const [, entry] of entries) {
		if (entry !== null) {
			entry?.size;
		}
	}
}

function forInObject(record: { a: 1 } | undefined) {
	for (const key in record) {
		record?.a;
	}
}

function forInReceiver(holder: { record?: { a: 1 } } | undefined) {
	for (const key in holder?.record) {
		holder?.record;
	}
}
