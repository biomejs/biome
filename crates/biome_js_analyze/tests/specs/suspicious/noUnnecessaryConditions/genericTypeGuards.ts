// should generate diagnostics

declare function isString(input: unknown): input is string;
declare function isNonNull<T>(input: T | null): input is T;
declare function generic<T>(input: unknown): input is string;
declare function overloaded(input: string): input is string;
declare function overloaded(input: unknown): input is string;

function genericGuard(value: string | null) {
	if (isNonNull(value)) value?.length;
}

function unusedTypeParameter(value: string | null) {
	if (generic(value)) value?.length;
}

function overloadedGuard(value: string | null) {
	if (overloaded(value)) value?.length;
}

function filteredElement(values: (string | null)[]) {
	return values.filter(isString).map((value) => value?.length);
}

function foundElement(values: (string | number)[]) {
	const found = values.find(isString);
	if (found) found?.length;
	return found?.length;
}
