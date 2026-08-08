/* should not generate diagnostics */

// An overloaded default export may be anonymous, so it binds no name.
// See https://github.com/biomejs/biome/issues/11214
export default function <T>(): void;
export default function <T>(value?: T): void {
	console.log(value);
}
