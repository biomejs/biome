// should generate diagnostics
// A string has the members of `String`, and a `readonly` array or tuple has
// the members of `ReadonlyArray`, as in TypeScript, so a check of them that
// can't fail is reported.

function stringLength(text: string) {
	if (text.length === undefined) {
		return 1;
	}
}

function readonlyTuple(pair: readonly [string, number]) {
	if (pair[0] === null) {
		return 1;
	}
}

function readonlyArrayLength(items: readonly string[]) {
	if (items.length === null) {
		return 1;
	}
}
