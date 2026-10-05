// should not generate diagnostics

declare function assert(condition: unknown): asserts condition;
declare function assertString(input: unknown): asserts input is string;
declare function assertFalse(input: unknown): asserts input is false;
declare function assertPromise(input: unknown): asserts input is Promise<void>;
declare function fail(input: unknown): never;
declare function consume(...values: unknown[]): void;
declare function identity<T>(input: T): T;

function typedPrimitive(value: Promise<void> | string) {
	assertString(value);
	value;
	identity(value);
}

function falsyTarget(value: Promise<void> | false) {
	assertFalse(value);
	value;
}

function bareCondition(value: Promise<void> | string) {
	assert(typeof value === "string");
	value;
}

function beforeAssertion(value: Promise<void> | null) {
	void value;
	assert(value);
	void value;
}

function oneBranch(value: Promise<void> | null, flag: boolean) {
	if (flag) assert(value);
	void value;
}

function loopMayNotRun(value: Promise<void> | null, flag: boolean) {
	while (flag) assert(value);
	void value;
}

function optional(value: Promise<void> | null) {
	assert?.(value);
	void value;
}

function nested(value: Promise<void> | null, flag: boolean) {
	consume(assert(value), value);
	flag && assert(value);
	flag ? assert(value) : undefined;
	void (assert(value), value);
	void assert(value);
	void value;
}

function assigned(value: Promise<void> | null) {
	const ignored = assert(value);
	void value;
}

async function handled(value: Promise<void> | null) {
	assert(value);
	await value;
}

function unsupportedTarget(value: unknown) {
	assertPromise(value);
	value;
}

function neverReturning(value: Promise<void> | null) {
	fail(value);
	void value;
}
