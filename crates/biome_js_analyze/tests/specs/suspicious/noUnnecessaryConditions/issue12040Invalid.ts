declare function identity<T>(value: T): T;
declare function pick<T>(value: T, label: string): T;

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
