// should generate diagnostics

function readonlyProperty(o: { readonly v: string | null }) {
	const ok = o.v !== null;
	if (ok) {
		return o.v?.length;
	}
}

function nestedPath(o: { readonly a: { readonly v: string | undefined } }) {
	const hasValue = o.a.v !== undefined;
	if (hasValue) {
		return o.a.v?.length;
	}
}

function readonlyMapped(o: Readonly<{ v: string | null }>) {
	const isString = typeof o.v === "string";
	if (isString) {
		return o.v?.length;
	}
}

class Cache {
	readonly value: string | undefined;

	constructor(value: string | undefined) {
		this.value = value;
	}

	read() {
		const ready = this.value !== undefined;
		if (ready) {
			return this.value?.length;
		}
	}
}
