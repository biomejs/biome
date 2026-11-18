// should generate diagnostics

function assertString(input: unknown, ignored?: unknown): asserts input is string {
	if (typeof input !== "string") throw new Error();
}

declare function assert(condition: unknown): asserts condition;
declare function assertFalse(input: unknown): asserts input is false;
declare function assertZero(input: unknown): asserts input is 0;
declare function assertEmpty(input: unknown): asserts input is "";
declare function assertNull(input: unknown): asserts input is null;
declare function second(this: void, ignored: unknown, input: unknown): asserts input is string;
declare function escaped(\u0076alue: unknown): asserts \u{76}alue is string;
declare function isString(input: unknown): input is string;
declare function identity<T>(input: T): T;

function standalone(value: string | null) {
	assertString(value);
	return value?.length;
}

function parenthesized(value: string | null) {
	(((assertString)((value))));
	return value ?? "fallback";
}

function truthy(value: "ready" | "" | false | 0 | null) {
	assert(value);
	if (value) {}
}

function falsyTargets(flag: boolean, number: 0 | 1, text: "" | "ready", nullable: string | null) {
	assertFalse(flag);
	if (flag) {}
	assertZero(number);
	if (number) {}
	assertEmpty(text);
	if (text) {}
	assertNull(nullable);
	if (nullable) {}
}

function conditions(value: string | number | null) {
	assert(value != null);
	assert(!(typeof value === "number"));
	return identity(value)?.length;
}

function predicate(value: string | number) {
	assert(isString(value));
	return value?.length;
}

function logical(value: string | number | null) {
	assert(value !== null && typeof value === "string");
	return value?.length;
}

function selectedArgument(value: string | null, other: unknown) {
	second(other, value);
	return value?.length;
}

function escapedParameter(value: string | null) {
	escaped(value);
	return value?.length;
}

function bothBranches(value: string | null, flag: boolean) {
	if (flag) assertString(value); else assertString(value);
	return value?.length;
}

function loop(value: string | null, flag: boolean) {
	do {
		assertString(value);
	} while (flag);
	return value?.length;
}
