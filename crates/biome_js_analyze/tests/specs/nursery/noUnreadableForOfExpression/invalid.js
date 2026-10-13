/* should generate diagnostics */

// Calls with complex arguments
for (const item of getItems(createArgument(seed))) {}
for (const item of getItems(argument || fallback)) {}
for (const item of getItems(...inputs)) {}
for (const item of getItems(tag`argument`)) {}
for (const pair of Iterator.zip([names, getScores(seed)])) {}
for (const pair of Iterator.zip([...inputs])) {}
for (const pair of Iterator.zip([[first], [second]])) {}
for (const pair of Iterator.zip([first, , second])) {}
for (const item of Object.keys({ a: 1 })) {}
for (const item of Object.keys(Object.fromEntries(entries))) {}
for (const item of Object.values(items.filter(item => item.isEnabled))) {}
for (
	const item of getItems(createArgument(seed))
) {}

async function asyncLoop() {
	for await (const item of getItems(createArgument(seed))) {}
	for (const item of await getItems()) {}
}

// Expressions that are neither a property access nor a call
for (const item of condition ? first : second) {}
for (const item of first || second) {}
for (const item of new Set(items)) {}
for (const item of tag`items`) {}
for (const character of `abc`) {}

// Complex objects and callees
for (const item of (header || "").split(",")) {}
for (const item of (condition ? first : second).values()) {}
for (const item of items.map(callback).values()) {}
for (const item of items[method]()) {}
for (const item of items[index]) {}
for (const item of getItems(seed)()) {}

// Optional chaining
for (const item of items?.values()) {}
for (const item of items?.entries) {}
for (const item of items?.map(item => item.value)) {}
for (const item of items?.map(callback)) {}
for (const item of items.filter?.(item => item.isEnabled)) {}
for (const item of items.filter?.(callback)) {}
for (const item of getItems?.()) {}

// Methods called on the result of a static `Iterator` method
for (const item of globalThis.Iterator.from(items).map(callback)) {}
for (const item of (globalThis).Iterator.from(items).map(callback)) {}
for (const item of (Iterator).from(items).map(callback)) {}
for (const item of Iterator.from(items).map(item => item.value)) {}
for (const item of Iterator.from(items).map(callback)) {}
for (const item of Iterator.from(items).filter(item => item.isEnabled)) {}
for (const item of Iterator.from(items).filter(callback)) {}
for (const item of Iterator.from(items).take(1)) {}
for (const item of Iterator.from(items).drop(1)) {}
for (const item of Iterator.from(items).flatMap(callback)) {}
for (const item of Iterator.concat(first, second).map(callback)) {}
for (const item of Iterator.zip(iterables).filter(callback)) {}
for (const item of Iterator.zipKeyed(iterables).take(1)) {}
for (const item of Iterator.from(items).take(1).map(callback)) {}

// Methods called on the result of a method that returns an iterator
for (const item of items.values().map(item => item.value)) {}
for (const item of items.values().map(callback)) {}
for (const item of items.values().filter(callback)) {}
for (const item of items.values().take(1)) {}
for (const item of items.values().take(1).map(callback)) {}
for (const item of items.keys().map(callback)) {}
for (const item of items.entries().filter(callback)) {}
for (const item of items.matchAll(pattern).map(callback)) {}
for (const item of items?.values().map(callback)) {}

// Methods called on a variable that holds an iterator
function iteratorVariable() {
	const iterator = items.values();
	for (const item of iterator.map(callback)) {}
}
function chainedIteratorVariables() {
	const iterator = items.values();
	const copy = iterator;
	for (const item of copy.take(1)) {}
}

// Methods called on the result of a generator function
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
	for (const item of generate().map(callback)) {}
}
function hoistedGenerator() {
	for (const item of generate().drop(1)) {}
	function* generate() {
		yield 1;
	}
}
export default function* exportedGenerator() {
	yield 1;
}
for (const item of exportedGenerator().take(1)) {}
