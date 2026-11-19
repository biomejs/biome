// should generate diagnostics
// A type predicate that targets `NonNullable<T>` takes the call's type
// argument, and `asserts this is T` gives `this` the intersection with `T`,
// so these values are never nullish.

declare function isNonNull<T>(value: T): value is NonNullable<T>;
declare function assertNonNull<T>(value: T): asserts value is NonNullable<T>;

function nonNullTarget(value: string | null) {
	if (isNonNull(value)) {
		return value ?? "";
	}
	return "";
}

function assertedTarget(value: string | undefined) {
	assertNonNull(value);
	return value ?? "";
}

class Ready {
	ready?: string;

	assertReady(): asserts this is { ready: string } {}

	label() {
		this.assertReady();
		return this.ready ?? "";
	}
}
