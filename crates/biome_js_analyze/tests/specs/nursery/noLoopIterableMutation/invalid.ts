/* should generate diagnostics */
for (const item of (items as string[])) {
	(items as string[]).push(item);
}

function deleteEntries(set: Set<[string]>) {
	for (const [value] of set) {
		set.delete(value);
	}
}

function reinsert(set: Set<string>) {
	for (const value of set!) {
		set!.delete(value);
		set!.add(value);
	}
}
