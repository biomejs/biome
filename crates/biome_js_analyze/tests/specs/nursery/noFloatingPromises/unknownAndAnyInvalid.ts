// should generate diagnostics
// `instanceof Promise` narrows `unknown` and `any` to `Promise<any>`, and a
// type-guard function narrows `any` to its target.

declare function isPromise(value: unknown): value is Promise<number>;

function unknownValue(value: unknown) {
	if (value instanceof Promise) {
		value;
	}
}

function anyValue(value: any) {
	if (value instanceof Promise) {
		value;
	}
}

function anyGuard(value: any) {
	if (isPromise(value)) {
		value;
	}
}
