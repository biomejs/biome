// should generate diagnostics
// `instanceof` keeps the union members derived from the class, so a check
// that the narrowed value can't fail is reported, as in TypeScript.

class Loaded {
	data = { size: 1 };
}

class Failed {
	data = null;
}

function removedMember(result: Loaded | Failed) {
	if (!(result instanceof Loaded)) {
		if (result.data) {
			return 1;
		}
	}
}

function optionalChain(result: Loaded | Failed | undefined) {
	if (result instanceof Loaded) {
		return result?.data.size;
	}
}

function falseBranch(result: Loaded | null) {
	if (!(result instanceof Loaded)) {
		if (result) {
			return 1;
		}
	}
}

function unknownValue(value: unknown) {
	if (value instanceof Error) {
		return value?.message;
	}
}

// An instance of a generic class has `any` type arguments, as `Map<any, any>`.
function genericClass(value: unknown) {
	if (value instanceof Map) {
		return value?.size;
	}
}
