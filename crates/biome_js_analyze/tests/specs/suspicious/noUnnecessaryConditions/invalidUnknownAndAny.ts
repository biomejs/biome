// should generate diagnostics
// A truthy or non-nullish `unknown` is `{}`, `typeof` with `"object"` narrows
// `unknown` to `object | null`, the `else` branch of a type guard for `{}`
// leaves `unknown` only `null | undefined`, and `any` narrows by `typeof` with
// a primitive kind, `instanceof`, and type-guard functions, also when it comes
// from an alias or from inference, so a check that the narrowed value can't
// fail is reported, as in TypeScript.

type Loose = any;

declare function isPresent(value: unknown): value is {};

function notNullish(value: unknown) {
	if (value != null) {
		return value ?? 1;
	}
}

function truthy(value: unknown) {
	if (value) {
		return value ?? 1;
	}
}

function objectNotNull(value: unknown) {
	if (typeof value === "object" && value !== null) {
		return value?.toString();
	}
}

function notPresent(value: unknown) {
	if (isPresent(value)) {
		return value;
	}
	if (value) {
		return value;
	}
}

function anyString(value: any) {
	if (typeof value === "string") {
		return value ?? "";
	}
}

function anyInstance(value: any) {
	if (value instanceof Error) {
		return value?.message;
	}
}

function anyArray(value: any) {
	if (Array.isArray(value)) {
		return value?.length;
	}
}

function aliasInstance(value: Loose) {
	if (value instanceof Error) {
		return value?.message;
	}
}

function parsedInstance(text: string) {
	const value = JSON.parse(text);
	if (value instanceof Error) {
		return value?.message;
	}
}
