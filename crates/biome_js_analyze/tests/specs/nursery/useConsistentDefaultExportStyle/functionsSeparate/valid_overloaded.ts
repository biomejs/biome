/* should not generate diagnostics */
export default function foo(value: string): string;
export default function foo(value: unknown) {
	return value;
}
