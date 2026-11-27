// should not generate diagnostics
// The `else` branch of a type-guard function whose target is an object type
// keeps the members that the target doesn't match, and `unknown`. Biome also
// keeps a member that it can't compare with the target, such as
// `{ size: number }` with the target `Cache`: a class can declare private
// members, and Biome types the field `size` as `0`, which TypeScript widens
// to `number`. TypeScript removes that member from the `else` branch.

interface Loaded {
	data: { size: number };
}

class Cache {
	size = 0;
}

declare function isLoaded(value: unknown): value is Loaded;
declare function isCache(value: unknown): value is Cache;

function falseBranch(result: Loaded | string | null) {
	if (!isLoaded(result)) {
		if (result) {
			return 1;
		}
	}
}

function unknownElse(value: unknown) {
	if (!isLoaded(value)) {
		return value ?? "fallback";
	}
}

function classTarget(value: { size: number } | null) {
	if (!isCache(value)) {
		return value?.size;
	}
}
