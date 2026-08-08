/* should not generate diagnostics */

// See https://github.com/biomejs/biome/issues/11214
export default function overloaded<T>(): void;
export default function overloaded<T>(value?: T): void {
	console.log(value);
}
