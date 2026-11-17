// should generate diagnostics
// Intersections are normalized as in TypeScript: `NonNullable<T>` is `T & {}`,
// `null` and `undefined` give `never` next to `{}` or an object type, and a
// property that two operands declare has the intersection of their types, so
// these values are never nullish.

class Options {
	label?: string;
}

function nonNullable(value: NonNullable<string | null>) {
	return value ?? "";
}

function distributed(value: (string | undefined) & {}) {
	return value ?? "";
}

function declaredTwice(options: Options & { label: string }) {
	return options.label ?? "";
}

function nullishWithObject(value: (Options | null) & { size: number }) {
	return value?.size;
}
