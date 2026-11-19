// should not generate diagnostics
// A property stays optional when every operand of an intersection declares it
// optional. A method that two operands declare has the union of both types,
// which is wider than the overloads TypeScript makes of them, and here gives
// the same values as the first overload, which TypeScript calls.

function bothOptional(options: { label?: string } & { label?: string | number }) {
	return options.label ?? "";
}

function method(value: { get(): string | undefined } & { get(): string }) {
	return value.get() ?? "";
}

function generic<T>(value: NonNullable<T> | undefined) {
	return value ?? "";
}
