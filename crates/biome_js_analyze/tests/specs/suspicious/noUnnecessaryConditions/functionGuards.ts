// should generate diagnostics

function isString(input: unknown): input is string {
	return typeof input === "string";
}

declare function isNull(input: unknown): input is null;
declare function isReady(input: unknown): input is "ready";
declare function secondString(this: void, ignored: unknown, input: unknown): input is string;
declare function escaped(\u0076alue: unknown): \u{76}alue is string;
declare function identity<T>(input: T): T;

function branches(value: "ready" | null) {
	if (isString(value)) {
		if (value) {}
	} else {
		if (value) {}
	}
}

function earlyReturn(value: string | null) {
	if (!isString(value)) return;
	return value?.length;
}

function falseBranch(value: string | null) {
	if (isNull(value)) return;
	return value ?? "fallback";
}

function literal(value: "ready" | "") {
	if (isReady(value)) {
		if (value) {}
	} else {
		if (value) {}
	}
}

function logical(value: string | null) {
	return isString(value) && value?.length;
}

function conditional(value: string | null) {
	return isString(value) ? value?.length : null;
}

function loop(value: string | null) {
	while (isString(value)) {
		value?.length;
	}
}

function secondArgument(value: string | null, other: unknown) {
	if (secondString(other, value)) {
		return value?.length;
	}
}

function escapedParameter(value: string | null) {
	if (escaped(value)) {
		return value?.length;
	}
}

function nestedCall(value: string | null) {
	if (isString(value)) {
		return identity(value)?.length;
	}
}
