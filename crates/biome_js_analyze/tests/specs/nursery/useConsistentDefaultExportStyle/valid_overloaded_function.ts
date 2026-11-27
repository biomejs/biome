/* should not generate diagnostics */
function foo(value: string): string;
function foo(value: number): number;
function foo(value: unknown) {
	return value;
}
export default foo;
