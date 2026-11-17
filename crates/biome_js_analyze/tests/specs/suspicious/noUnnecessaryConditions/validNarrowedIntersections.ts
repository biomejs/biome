// should not generate diagnostics
// Narrowing keeps the values that a test leaves possible: `!== null` keeps
// `undefined` in a value of a type parameter, `typeof` with `"object"` keeps
// `null`, and a comparison with a literal keeps the type parameter.

function notNull<T>(value: T, fallback: T) {
	if (value !== null) {
		return value ?? fallback;
	}
	return fallback;
}

function objectKind<T>(value: T) {
	if (typeof value === "object") {
		return value ?? {};
	}
	return {};
}

function literal<T>(value: T, fallback: T) {
	if (value === "a") {
		return value ?? fallback;
	}
	return fallback;
}

function falsy<T>(value: T, fallback: T) {
	if (!value) {
		return value ?? fallback;
	}
	return fallback;
}
