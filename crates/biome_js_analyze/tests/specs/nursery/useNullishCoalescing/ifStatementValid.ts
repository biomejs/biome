/* should not generate diagnostics */

declare function makeObj(): { x: number };
declare function sideEffect(): void;

// `=== null` on a union that also has `undefined`
declare let maybe: string | null | undefined;
if (maybe === null) {
	maybe = 'default';
}

// `=== undefined` on a union that also has `null`
declare let undefCheck: string | null | undefined;
if (undefCheck === undefined) {
	undefCheck = 'default';
}

// Compound check with the same literal twice
declare let vacuous: string | null | undefined;
if (vacuous === null || vacuous === null) {
	vacuous = 'default';
}

// Optional property that also accepts `null`
declare let input: { fruits?: string[] | null };
if (input.fruits === null) {
	input.fruits = [];
}

// `else` branch
declare let withElse: { x: number } | null;
if (!withElse) {
	withElse = makeObj();
} else {
	withElse = makeObj();
}

// `!x` on a non-nullish value
declare let notNullish: { x: number };
if (!notNullish) {
	notNullish = makeObj();
}

// More than one statement in the body
declare let multi: { x: number } | null;
if (!multi) {
	multi = makeObj();
	sideEffect();
}

// Test and body with different references
declare let mismatchA: { x: number } | null;
declare let mismatchB: { x: number } | null;
if (!mismatchA) {
	mismatchB = makeObj();
}

// Calls
declare function getRec(): { a: number | null };
if (getRec().a == null) {
	getRec().a = 1;
}

if (getRec()['a'] == null) {
	getRec()['a'] = 1;
}

// Compound side comparing the property with itself
declare let selfCompared: { a: string | null };
if (selfCompared.a === selfCompared.a || selfCompared.a === null) {
	selfCompared.a = 'v';
}

// Compound assignment that combines the old value
declare let counter: number | null;
if (counter == null) {
	counter += 1;
}

// `&&=` only assigns when the value is present
declare let andAssign: string | null;
if (andAssign == null) {
	andAssign &&= 'x';
}

// Test that checks the value is present
declare let present: string | null | undefined;
if (present != null) {
	present = 'x';
}
if (present !== null) {
	present = 'x';
}
if (present !== undefined) {
	present = 'x';
}

// Test and target read different properties or objects
declare let recMismatch: { a: number | null; b: number | null };
declare let recA: { a: number | null };
declare let recB: { a: number | null };
declare let record: Record<string, number | null>;
declare let key1: string;
declare let key2: string;
declare let elements: (number | null)[];
if (recMismatch.a == null) {
	recMismatch.b = 1;
}
if (recA.a == null) {
	recB.a = 1;
}
if (record[key1] == null) {
	record[key2] = 1;
}
if (elements[0] == null) {
	elements[1] = 1;
}

// Targets that are not a variable or a property
declare let wrappedTarget: string | null;
declare let destructured: string | null;
declare let sequenced: string | null;
declare let other: number;
declare const source: { destructured: string | null };
if (wrappedTarget == null) {
	(wrappedTarget as string) = 'v';
}
if (destructured == null) {
	[destructured] = ['v'];
}
if (destructured == null) {
	({ destructured } = source);
}
if (sequenced == null) {
	(sequenced = 'v'), (other = 1);
}

// Type assertion that removes `null` from the tested value
declare let pinned: string | null;
if ((pinned as string) == null) pinned = 'v';
