// should generate diagnostics
// `declare global` adds members to the global types, as in TypeScript, so a
// method that it declares returns its declared type, which isn't nullish.

declare global {
	interface String {
		format(): string;
	}
}

function wrapper(text: string) {
	return text.format() ?? "unformatted";
}

export {};
