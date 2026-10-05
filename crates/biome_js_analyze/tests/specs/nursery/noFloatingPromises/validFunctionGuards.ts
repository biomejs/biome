// should not generate diagnostics

function isString(input: unknown): input is string {
	return typeof input === "string";
}

declare function isNull(input: unknown): input is null;
declare function isUndefined(input: unknown): input is undefined;
declare function isFalse(input: unknown): input is false;
declare function identity<T>(input: T): T;
declare function structural(input: unknown): input is Promise<void>;
declare function assertPromise(input: unknown): asserts input is Promise<void>;

function stringBranch(value: Promise<void> | string) {
	if (isString(value)) {
		value;
		identity(value);
	}
}

function earlyReturn(value: Promise<void> | string) {
	if (!isString(value)) return;
	value;
}

function nullBranch(value: Promise<void> | null) {
	if (isNull(value)) {
		value;
	}
}

function undefinedBranch(value: Promise<void> | undefined) {
	if (isUndefined(value)) {
		value;
	}
}

function literalBranch(value: Promise<void> | false) {
	if (isFalse(value)) {
		value;
	}
}

function nullArray(value: Promise<void>[] | null) {
	if (isNull(value)) {
		value;
	}
}

async function handled(value: Promise<void> | null) {
	if (isNull(value)) return;
	await value;
}

function unsupportedTarget(value: unknown) {
	if (structural(value)) {
		value;
	}
	assertPromise(value);
	value;
}
