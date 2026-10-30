declare function identity<T>(value: T): T;
declare function pick<T>(value: T, label: string): T;
declare function either<T>(first: T, second: T): T;
declare function maybe<T>(first: T, second?: T): T;

export function identityLoading() {
	const loading = identity(undefined);

	if (loading) {
		return loading;
	}
}

export function pickedLoading() {
	const loading = pick(undefined, "loading");

	if (loading) {
		return loading;
	}
}

export function eitherLoading() {
	const loading = either(undefined, undefined);

	if (loading) {
		return loading;
	}
}

export function maybeLoading() {
	const loading = maybe(undefined);

	if (loading) {
		return loading;
	}
}
