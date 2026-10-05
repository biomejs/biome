// should generate diagnostics

function assert(condition: unknown): asserts condition {
	if (!condition) throw new Error();
}

declare function isString(input: unknown): input is string;
declare function identity<T>(input: T): T;

function truthy(value: Promise<void> | null | false) {
	assert(value);
	value;
}

function condition(value: Promise<void> | null) {
	assert(value !== null);
	identity(value);
}

function predicate(value: Promise<void> | string) {
	assert(!isString(value));
	value;
}

function multiple(value: Promise<void> | string | null) {
	assert(value != null);
	assert(typeof value !== "string");
	identity(value);
}

function bothBranches(value: Promise<void> | null, flag: boolean) {
	if (flag) assert(value); else assert(value != null);
	value;
}

function loop(value: Promise<void> | null, flag: boolean) {
	do {
		assert(value);
	} while (flag);
	value;
}

function array(value: Promise<void>[] | null) {
	assert(value != null);
	value;
}

function callback(value: (() => Promise<void>) | null) {
	assert(value);
	value();
}
