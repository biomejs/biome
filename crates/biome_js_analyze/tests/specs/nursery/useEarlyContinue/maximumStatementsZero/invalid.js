/* should generate diagnostics */
for (const item of items) {
	if (item.isActive) {
		process(item);
	}
}

for (const item of items) {
	if (item.isActive)
		process(item);
}

for (const item of items) {
	prepare(item);
	if (item.isActive) process(item);
}

for (const item of items) { if (item.isActive) process(item); }

for (const item of items) {
	if (item.isActive) {
		if (item.isReady) {
			process(item);
		}
	}
}
