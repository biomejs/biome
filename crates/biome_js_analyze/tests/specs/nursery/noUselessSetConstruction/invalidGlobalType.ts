/* should generate diagnostics */
export {};

// A declaration in `declare global` describes the built-in `Set` and `Map`
declare global {
	var Set: SetConstructor;
	interface Set<T> {
		extra: T;
	}
	interface ReadonlyMap<K, V> {
		extra: [K, V];
	}
}

function globalTypes(selected: Set<number>, records: ReadonlyMap<number, string>) {
	return selected.union(new Set(records.keys()));
}
