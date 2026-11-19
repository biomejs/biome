// should generate diagnostics
// `!== null` narrows `unknown` to `{} | undefined`, which can be nullish.

function strictNull(value: unknown) {
	if (value !== null) {
		return value || 1;
	}
}
