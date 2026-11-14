// should generate diagnostics
// A later declaration of a merged interface gives the overload that a call
// tries first, as in TypeScript, and `declare global` adds methods to the
// global types, so these calls return promises.

interface Store {
	save(): void;
}

interface Store {
	save(): Promise<void>;
}

declare const store: Store;
store.save();

declare global {
	interface Array<T> {
		flush(): Promise<void>;
	}
}

declare const items: string[];
items.flush();

export {};
