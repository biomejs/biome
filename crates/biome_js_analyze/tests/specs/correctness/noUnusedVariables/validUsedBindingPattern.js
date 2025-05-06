/* should not generate diagnostics */
export function f({ a, b = a }) {
	console.info(b);
}
