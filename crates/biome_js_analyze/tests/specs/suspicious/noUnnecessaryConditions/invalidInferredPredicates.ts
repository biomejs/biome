// should generate diagnostics
// A function without a return type annotation whose body returns a test of
// a parameter has the type predicate that TypeScript infers from it, so a
// call narrows its argument, and `filter` with such a callback keeps only the
// elements that pass, as in TypeScript.

const isPresent = (value: string | null) => value !== null;

function narrowedArgument(value: string | null) {
	if (isPresent(value)) {
		return value ?? "";
	}
}

function narrowedAfterReturn(value: string | null) {
	if (!isPresent(value)) {
		return "";
	}
	return value ?? "";
}

function filteredElements(values: (string | null)[]) {
	return values.filter((value) => value !== null).map((value) => value ?? "");
}
