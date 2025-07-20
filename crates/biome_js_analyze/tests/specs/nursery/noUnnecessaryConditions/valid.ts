/* should not generate diagnostics */


declare const b1: boolean;
declare const b2: boolean;
const t1 = b1 && b2;
const t2 = b1 || b2;
if (b1 && b2) {
}
while (b1 && b2) { }
for (let i = 0; b1 && b2; i++) {
	break;
}
const t1 = b1 && b2 ? 'yes' : 'no';
if (b1 && b2) {
}
while (b1 && b2) { }
for (let i = 0; b1 && b2; i++) {
	break;
}
const t1 = b1 && b2 ? 'yes' : 'no';
for (; ;) { }
switch (b1) {
	case true:
	default:
}

declare const b1: boolean;
declare const b2: true;
const x = b1 && b2;















//Incorrect

function head<T>(items: T[]) {
	// items can never be nullable, so this is unnecessary
	if (items) {
			return items[0].toUpperCase();
	}
}

function foo(arg: 'bar' | 'baz') {
	// arg is never nullable or empty string, so this is unnecessary
	if (arg) {
	}
}

function bar<T>(arg: string) {
	// arg can never be nullish, so ?. is unnecessary
	return arg?.length;
}

// Checks array predicate return types, where possible
[
	[1, 2],
	[3, 4],
].filter(t => t); // number[] is always truthy

//	Correct

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
	// ...
}

for (; true; ) {
	// ...
}

do {
	// ...
} while (true);

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
	// ...
}

// not even a variable that references the value of `true` is allowed, only
// the literal value of `true` used directly.

const thisIsTrue = true;

while (thisIsTrue) {
	// ...
}

Options: { "checkTypePredicates": true }

function assert(condition: unknown): asserts condition {
	if (!condition) {
		throw new Error('Condition is falsy');
	}
}

assert(false); // Unnecessary; condition is always falsy.

	const neverNull = {};
assert(neverNull); // Unnecessary; condition is always truthy.
~~~~~~~~~ Unnecessary conditional, value is always truthy.

	function isString(value: unknown): value is string {
	return typeof value === 'string';
}

declare const s: string;

// Unnecessary; s is always a string.
if (isString(s)) {
}

function assertIsString(value: unknown): asserts value is string {
	if (!isString(value)) {
		throw new Error('Value is not a string');
	}
}

assertIsString(s); // Unnecessary; s is always a string.

const array: string[] = [];
const firstElement = array[0];
// false positive
if (firstElement != null) {
	// ...
}

const record: Record<string, string> = {};
const someValue = record.someKey;
// false positive
if (someValue != null) {
	// ...
}



let condition = false;

const f = () => {
	condition = Math.random() > 0.5;
};
f();

if (condition) {
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