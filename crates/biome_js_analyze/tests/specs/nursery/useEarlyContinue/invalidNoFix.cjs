/* should generate diagnostics */
for (const item of items) {
	if (item.isActive) {
		label: function helper() {}
		helper(item);
	}
}

for (const item of items) {
	if (item.isActive) {
		outer: inner: function helper() {}
		helper(item);
	}
}
