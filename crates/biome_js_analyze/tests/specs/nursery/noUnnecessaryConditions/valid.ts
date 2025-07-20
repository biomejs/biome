/* should not generate diagnostics */

Incorrect

function head<T>(items: T[]) {
	// items can never be nullable, so this is unnecessary
	if (items) {
		~~~~~ Unnecessary conditional, value is always truthy.
			return items[0].toUpperCase();
	}
}

function foo(arg: 'bar' | 'baz') {
	// arg is never nullable or empty string, so this is unnecessary
	if (arg) {
		~~~ Unnecessary conditional, value is always truthy.
	}
}

function bar<T>(arg: string) {
	// arg can never be nullish, so ?. is unnecessary
	return arg?.length;
	~~ Unnecessary optional chain on a non-nullish value.
}

// Checks array predicate return types, where possible
[
	[1, 2],
	[3, 4],
].filter(t => t); // number[] is always truthy
~ Unnecessary conditional, value is always truthy.

	Correct

function head<T>(items: T[]) {
	// Necessary, since items.length might be 0
	if (items.length) {
		return items[0].toUpperCase();
	}
}

function foo(arg: string) {
	// Necessary, since arg might be ''.
	if (arg) {
	}
}

function bar(arg?: string | null) {
	// Necessary, since arg might be nullish
	return arg?.length;
}

[0, 1, 2, 3].filter(t => t); // number can be truthy or falsy

Options: { "allowConstantLoopConditions": "never" }

while (true) {
	~~~~ Unnecessary conditional, value is always truthy.
	// ...
}

for (; true; ) {
	~~~~ Unnecessary conditional, value is always truthy.
	// ...
}

do {
	// ...
} while (true);
~~~~ Unnecessary conditional, value is always truthy.

	Options: { "allowConstantLoopConditions": "always" }

while (true) {
	// ...
}

for (; true; ) {
	// ...
}

do {
	// ...
} while (true);

Options: { "allowConstantLoopConditions": "only-allowed-literals" }

while (true) {
	// ...
}

Options: { "allowConstantLoopConditions": "only-allowed-literals" }

// `alwaysTrue` has the type of `true` (which isn't allowed)
// as only the literal value of `true` is allowed.

declare const alwaysTrue: true;

while (alwaysTrue) {
	~~~~~~~~~~ Unnecessary conditional, value is always truthy.
	// ...
}

// not even a variable that references the value of `true` is allowed, only
// the literal value of `true` used directly.

const thisIsTrue = true;

while (thisIsTrue) {
	~~~~~~~~~~ Unnecessary conditional, value is always truthy.
	// ...
}

Options: { "checkTypePredicates": true }

function assert(condition: unknown): asserts condition {
	if (!condition) {
		throw new Error('Condition is falsy');
	}
}

assert(false); // Unnecessary; condition is always falsy.
~~~~~ Unnecessary conditional, value is always falsy.

	const neverNull = {};
assert(neverNull); // Unnecessary; condition is always truthy.
~~~~~~~~~ Unnecessary conditional, value is always truthy.

	function isString(value: unknown): value is string {
	return typeof value === 'string';
}

declare const s: string;

// Unnecessary; s is always a string.
if (isString(s)) {
	~ Unnecessary conditional, expression already has the type being checked by the type guard.
}

function assertIsString(value: unknown): asserts value is string {
	if (!isString(value)) {
		throw new Error('Value is not a string');
	}
}

assertIsString(s); // Unnecessary; s is always a string.
~ Unnecessary conditional, expression already has the type being checked by the assertion function.



const array: string[] = [];
const firstElement = array[0];
// false positive
if (firstElement != null) {
	~~~~~~~~~~~~~~~~~~~~ Unnecessary conditional, the types have no overlap.
	// ...
}

const record: Record<string, string> = {};
const someValue = record.someKey;
// false positive
if (someValue != null) {
	~~~~~~~~~~~~~~~~~ Unnecessary conditional, the types have no overlap.
	// ...
}



let condition = false;

const f = () => {
	condition = Math.random() > 0.5;
};
f();

if (condition) {
	~~~~~~~~~ Unnecessary conditional, value is always falsy.
	// ...
}



let condition = false as boolean;

const f = () => {
	condition = Math.random() > 0.5;
};
f();

if (condition) {
	// ...
}