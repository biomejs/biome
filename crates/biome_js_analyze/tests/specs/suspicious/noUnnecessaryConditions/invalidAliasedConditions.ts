// should generate diagnostics

function notNull(value: string | null) {
	const ok = value !== null;
	if (ok) {
		return value?.length;
	}
}

function earlyReturn(value: string | number | undefined) {
	const isString = typeof value === "string";
	if (!isString) return;
	return value?.length;
}

function both(a: string | null, b: string | null) {
	const ok = a !== null && b !== null;
	if (ok) {
		return b?.length;
	}
}

function chained(value: string | null) {
	const ok = value !== null;
	const ready = ok;
	if (ready) {
		return value?.length;
	}
}

function receiver(o: { v: string } | undefined) {
	const hasValue = o?.v;
	if (hasValue) {
		return o?.v;
	}
}
