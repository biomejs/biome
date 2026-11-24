// should generate diagnostics
// A type-guard function or an assertion function whose target is an object
// type or a union narrows the tested value, as in TypeScript, so a check
// that the narrowed value can't fail is reported.

interface Loaded {
	data: { size: number };
}

interface Failed {
	error: string;
}

declare function isLoaded(value: unknown): value is Loaded;
declare function isLoadedOrFailed(value: unknown): value is Loaded | Failed;
declare function isText(value: unknown): value is string | number;
declare function isNamed(value: unknown): value is { name: string };
declare function assertLoaded(value: unknown): asserts value is Loaded;
declare function assertText(value: unknown): asserts value is string | number;

const isLoadedArrow = (value: unknown): value is Loaded => typeof value === "object";

function trueBranch(result: Loaded | undefined) {
	if (isLoaded(result)) {
		return result?.data.size;
	}
}

function falseBranch(result: Loaded | null) {
	if (!isLoaded(result)) {
		if (result) {
			return 1;
		}
	}
}

function unknownValue(value: unknown) {
	if (isNamed(value)) {
		return value?.name;
	}
}

function unionTarget(value: Loaded | Failed | null) {
	if (isLoadedOrFailed(value)) {
		return value ?? "fallback";
	}
}

function primitiveUnionTarget(value: unknown) {
	if (isText(value)) {
		return value ?? "fallback";
	}
}

function arrow(result: Loaded | undefined) {
	if (isLoadedArrow(result)) {
		return result?.data;
	}
}

function assertion(value: unknown) {
	assertLoaded(value);
	return value?.data;
}

function unionAssertion(value: unknown) {
	assertText(value);
	return value ?? "fallback";
}
