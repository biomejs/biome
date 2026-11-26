/* should generate diagnostics */
for (const item of items) {
	if (item.isActive) {
		function helper() {}
		helper(item);
	}
}

for (const item of items) {
	if (item.isActive) {
		class Helper {}
		new Helper(item);
	}
}

for (const item of items) {
	prepare(item);
	if (item.isActive) {
		const value = item.value;
		process(value);
	}
}

for (const item of items) {
	if (value) {
		const value = item.value;
		process(value);
	}
}

for (const item of items) {
	if (check(() => value)) {
		const { value } = item;
		process(value);
	}
}

for (const item of items) {
	if (eval("item.isActive")) {
		let value = item.value;
		process(value);
	}
}

for (const item of items) {
	if (item.isActive) { // Only active items.
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item.isActive) {
		process(item);
		save(item);
		// Done.
	}
}

for (const item of items) {
	if (count++ < 10) {
		const count = item.count;
		log(count);
	}
}

for (const item of items) {
	if ((total = item.total)) {
		let total = 0;
		log(total);
	}
}
