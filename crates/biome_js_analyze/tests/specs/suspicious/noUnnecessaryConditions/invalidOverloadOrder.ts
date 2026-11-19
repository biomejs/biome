// should generate diagnostics
// The overloads of one declaration with a parameter of a literal type are
// tried first for a call, as in TypeScript, so the call has the type that the
// first matching one of them returns.

interface Store {
	get(key: string): string | undefined;
	get(key: "name"): string;
}

function storedName(store: Store) {
	return store.get("name") ?? "anonymous";
}

declare function setting(key: string): string | undefined;
declare function setting(key: "mode"): "light" | "dark";

function mode() {
	return setting("mode") ?? "light";
}
