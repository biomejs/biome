// should generate diagnostics

declare function isString(input: unknown): input is string;
declare function isNull(input: unknown): input is null;
declare function identity<T>(input: T): T;

function falseBranch(value: Promise<void> | string) {
	if (isString(value)) {
		return;
	} else {
		value;
	}
}

function negated(value: Promise<void> | string) {
	if (!isString(value)) {
		value;
	}
}

function earlyReturn(value: Promise<void> | null) {
	if (isNull(value)) return;
	value;
}

function logical(value: Promise<void> | string) {
	isString(value) || value;
}

function conditional(value: Promise<void> | string) {
	isString(value) ? undefined : value;
}

function nestedCall(value: Promise<void> | string) {
	if (isString(value)) return;
	identity(value);
}

function array(value: Promise<void>[] | null) {
	if (isNull(value)) return;
	value;
}

function callback(value: (() => Promise<void>) | null) {
	if (isNull(value)) return;
	value();
}
