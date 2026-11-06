/* should generate diagnostics */
for (const item of items.map(callback)) {}
for (const item of items.filter(callback)) {}
for (const item of items.map(callback, thisArgument)) {}
for (const item of items.filter(callback, thisArgument)) {}
for (const item of items.map(...callbacks)) {}
for (const item of (items).map(callback)) {}
for (const item of ((items)).filter(callback)) {}
for (const item of (items.map(callback))) {}
for (const item of (0, items.map(callback))) {}
for await (const item of items.map(callback)) {}
for await (const item of items.filter(async (item) => item.isEnabled)) {}
for (const item of items.map(/* comment */ callback)) {}
for (const item of items
	.map((item) => item.value)
) {}
for (
	const item of items.filter((item) => item.isEnabled)
) {}

// Only the reported call itself has to be free of optional chaining.
for (const item of items?.children.map(callback)) {}

// Calls that don't return an iterator.
for (const item of items.values(1).map(callback)) {}
for (const item of items.matchAll(pattern, extra).filter(callback)) {}
for (const item of Iterator.from(items).take().map(callback)) {}
for (const item of items.slice(1).map(callback)) {}

function shadowedIteratorFrom() {
	const Iterator = { from: (items) => items };
	for (const item of Iterator.from(items).map(callback)) {}
}

function shadowedIteratorZip() {
	const Iterator = { zip: (iterables) => iterables };
	for (const item of Iterator.zip(iterables).filter(callback)) {}
}

function shadowedGlobalThis() {
	const globalThis = { Iterator: { from: (items) => items } };
	for (const item of globalThis.Iterator.from(items).map(callback)) {}
}

function constantArray() {
	const list = [1, 2, 3];
	for (const item of list.map(callback)) {}
}

function reassignedVariable() {
	let list = items.values();
	list = [...list];
	for (const item of list.map(callback)) {}
}

function destructuredConstant() {
	const { list } = source;
	for (const item of list.map(callback)) {}
}

function parameter(list) {
	for (const item of list.map(callback)) {}
}

function plainFunctionCall() {
	function getItems() {
		return [];
	}
	for (const item of getItems().map(callback)) {}
}

function reassignedGenerator() {
	function* generate() {
		yield 1;
	}
	generate = getItems;
	for (const item of generate().filter(callback)) {}
}

function constantsReferringToEachOther() {
	const first = second;
	const second = first;
	for (const item of first.map(callback)) {}
	for (const item of first().filter(callback)) {}
}
