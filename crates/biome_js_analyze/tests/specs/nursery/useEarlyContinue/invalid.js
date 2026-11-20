/* should generate diagnostics */
for (const item of items) {
	if (item.isActive) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	doSomethingBefore();
	if (item.isActive) {
		process(item);
		save(item);
	}
}

for (let index = 0; index < items.length; index++) {
	if (items[index].isActive) {
		process(items[index]);
		save(items[index]);
	}
}

for (const key in items) {
	if (items[key].isActive) {
		process(items[key]);
		save(items[key]);
	}
}

while ((item = items.shift())) {
	if (item.isActive) {
		process(item);
		save(item);
	}
}

do {
	if (item.isActive) {
		process(item);
		save(item);
	}
} while (next());

async function run() {
	for await (const item of items) {
		if (item.isActive) {
			process(item);
			save(item);
		}
	}
}

for (const item of items) {
	if (!item.isActive) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (!(item.isActive && item.isReady)) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item.isActive || item.isPending) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (item.count > 0) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if ((item.isActive)) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (check(() => item)) {
		process(item);
		save(item);
	}
}

function run(items) {
	for (const item of items) {
		if (item.isActive) {
			if (item.isReady) {
				return;
			}

			process(item);
			save(item);
		}
	}
}

function run(items) {
	for (const item of items) {
		if (item.isActive) {
			process(item);
			if (item.isDone) {
				return;
			}
		}
	}
}

for (const item of items) {
	if (item.isActive) {
		const value = item.value;
		process(value);
	}
}

for (const item of items) {
	if (item.isActive) {
		var value = item.value;
		process(value);
	}
}

for (const item of items) {
	// Only active items.
	if (item.isActive) {
		// Process first.
		process(item);
		save(item); // Then save.
	}
}

for (const item of items) {
	if (/* active */ item.isActive) {
		process(item);
		save(item);
	}
}

for (const item of items) {
	if (!/* active */ item.isActive) {
		process(item);
		save(item);
	}
}

for (const item of items) { if (item.isActive) { process(item); save(item); } }

outer: for (const item of items) {
	if (item.isActive) {
		inner: for (const child of item.children) {
			process(child);
		}
		save(item);
	}
}
