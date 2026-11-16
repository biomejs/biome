// should not generate diagnostics

declare function isString(input: unknown): input is string;
declare function read(): string | null;
declare function generic<T>(input: unknown): input is string;
declare function overloaded(input: string): input is string;
declare function overloaded(input: unknown): input is string;

function beforeAndAfterJoin(value: string | null) {
	value?.length;
	if (isString(value)) {
		value.length;
	}
	return value?.length;
}

function callArgument(value: string | null) {
	if (isString(value?.trim())) {
		return value?.length;
	}
}

function stringMayBeEmpty(value: string | number) {
	if (isString(value)) {
		if (value) {}
	}
}

function shadowedSubject(value: string | null) {
	if (isString(value)) {
		const value = read();
		return value?.length;
	}
}

function written(value: string | null) {
	if (isString(value)) {
		value = read();
		return value?.length;
	}
}

function unsupportedCallees(value: string | null) {
	if (generic(value)) value?.length;
	if (overloaded(value)) value?.length;
	return value?.length;
}

function optionalCalleeThatIsFalsy(value: string | null, guard: ((input: unknown) => input is string) | undefined) {
	if (!guard?.(value)) {
		return value?.length;
	}
}

function guardWithoutDottedCallee(value: string | null, guards: Array<(input: unknown) => input is string>) {
	if (guards[0](value)) {
		return value?.length;
	}
}
