// should not generate diagnostics

declare function get(): string | null;

function annotated(value: string | null) {
	const ok: boolean = value !== null;
	if (ok) {
		return value?.length;
	}
}

function mutableAlias(value: string | null) {
	let ok = value !== null;
	if (ok) {
		return value?.length;
	}
}

function either(value: string | null, other: boolean) {
	const ok = value !== null;
	if (ok || other) {
		return value?.length;
	}
}

function written(value: string | null) {
	let current = value;
	const ok = current !== null;
	current = get();
	if (ok) {
		return current?.length;
	}
}

function mutableProperty(o: { v: string | null }) {
	const ok = o.v !== null;
	if (ok) {
		return o.v?.length;
	}
}

function tooDeep(value: string | null) {
	const c1 = value !== null;
	const c2 = c1;
	const c3 = c2;
	const c4 = c3;
	const c5 = c4;
	const c6 = c5;
	if (c6) {
		return value?.length;
	}
}
