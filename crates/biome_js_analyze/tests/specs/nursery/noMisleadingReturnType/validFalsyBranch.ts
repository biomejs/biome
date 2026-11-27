// should not generate diagnostics
// TypeScript reads `value` as `string` inside `if (!value)`.

function emptyString(value: string): string {
	if (!value) {
		return value;
	}
	return "a";
}
