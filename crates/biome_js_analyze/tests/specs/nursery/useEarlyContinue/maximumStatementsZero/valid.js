/* should not generate diagnostics */
for (const item of items) {
	if (item.isActive);
}

for (const item of items) {
	if (item.isActive) {}
}

function run(items) {
	for (const item of items) {
		if (item.isActive) return;
	}
}

for (const item of items) {
	if (item.isActive) break;
}

for (const item of items) {
	if (item.isActive) throw new Error("Active item");
}

for (const item of items) {
	if (item.isActive) continue;
}

for (const item of items) {
	if (item.isActive) {
		continue;
		;
	}
}
