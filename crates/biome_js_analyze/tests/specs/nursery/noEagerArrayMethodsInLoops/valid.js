/* should not generate diagnostics */
for (const item of items) {}
for await (const item of items) {}
for (const key in items.map(callback)) {}
items.map(callback);
items.filter(callback);
for (const item of items.map()) {}
for (const item of items.filter()) {}
for (const item of items?.map(callback)) {}
for (const item of items.map?.(callback)) {}
for (const item of items["map"](callback)) {}
for (const item of items[method](callback)) {}
for (const item of items.flatMap(callback)) {}
for (const [index, item] of items.entries()) {}
for (const [index, item] of items.map(callback).entries()) {}
for (const [index, item] of items.filter(callback).entries()) {}
for (const item of Array.from(items)) {}
for (const item of [...items]) {}
for (const item of Array.prototype.map.call(items, callback)) {}

// Static methods of the global `Iterator` object.
for (const item of Iterator.from(items).map(callback)) {}
for (const item of Iterator.from(items).filter(callback)) {}
for (const item of Iterator.from(items).map(callback).filter(callback)) {}
for (const item of (Iterator).from(items).map(callback)) {}
for (const item of Iterator.concat(first, second).filter(callback)) {}
for (const item of Iterator.zip(iterables).map(callback)) {}
for (const item of Iterator.zipKeyed(iterables).filter(callback)) {}
for (const item of globalThis.Iterator.from(items).map(callback)) {}
for (const item of (globalThis).Iterator.from(items).map(callback)) {}
for (const item of globalThis.Iterator.zip(iterables).filter(callback)) {}
for (const item of window.Iterator.from(items).map(callback)) {}

// Methods that return an iterator.
for (const item of items.values().map(callback)) {}
for (const item of items.keys().map(callback)) {}
for (const item of items.entries().filter(callback)) {}
for (const item of items?.values().map(callback)) {}
for (const item of items.matchAll(pattern).map(callback)) {}
for (const item of Iterator.from(items).take(1).map(callback)) {}
for (const item of Iterator.from(items).flatMap(callback).drop(1).filter(callback)) {}

function precomputedArray() {
	const mapped = items.map(callback);
	for (const item of mapped) {}
}

function constantIterator() {
	const iterator = items.values();
	for (const item of iterator.map(callback)) {}
}

function constantIteratorAlias() {
	const iterator = items.values();
	const alias = iterator;
	for (const item of alias.filter(callback)) {}
}

function generatorDeclaration() {
	function* generate() {
		yield 1;
	}
	for (const item of generate().filter(callback)) {}
}

function generatorExpression() {
	const generate = function* () {
		yield 1;
	};
	const alias = generate;
	for (const item of generate().map(callback)) {}
	for (const item of alias().filter(callback)) {}
}

function constantGeneratorResult() {
	function* generate() {
		yield 1;
	}
	const iterator = generate();
	for (const item of iterator.map(callback)) {}
}

async function asyncGeneratorDeclaration() {
	async function* generate() {
		yield 1;
	}
	for await (const item of generate().map(callback)) {}
}

export default function* generateDefault() {
	yield 1;
}
for (const item of generateDefault().map(callback)) {}
