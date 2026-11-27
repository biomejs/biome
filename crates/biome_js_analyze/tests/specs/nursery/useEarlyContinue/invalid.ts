/* should generate diagnostics */
for (const item of items) {
	if (item as ActiveItem) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item!) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item satisfies ActiveItem) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item.isActive) {
		type Item = typeof item;
		process(item as Item);
	}
}

for (const item of items) {
	if (item.isActive) {
		await using resource = open(item);
		process(resource);
	}
}

for (const item of items) {
	if (item.isActive) {
		enum Kind { A }
		process(Kind.A);
	}
}
