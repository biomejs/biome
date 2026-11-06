/* should generate diagnostics */
function parameter(value: number) {
	return value < 0 ? -value : value;
}
function satisfiesAssertion(value) {
	return (value satisfies number) < 0 ? -(value satisfies number) : (value satisfies number);
}
function nonNullAssertion(value?: number) {
	return value! < 0 ? -value! : value!;
}
function range(value: number, limit: unknown) {
	return value > (limit as number) || value < -(limit as number);
}
function rangeWithoutParentheses(value: number, limit: unknown) {
	return limit as number < value || -(limit as number) > value;
}
function rangeSatisfies(value: number, limit: number) {
	return limit satisfies number <= value || -limit >= value;
}
