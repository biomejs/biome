/* should generate diagnostics */
for (const item of items) {
	if (item.isActive) {
		process(item);
		save(item);
		log(item);
	}
}
