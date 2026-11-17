// should not generate diagnostics
// A comparison of a condition with `false` narrows where the condition isn't
// `false` only when every falsy value of the condition is `false`, a loose
// comparison with a value that may be `null` keeps `null`, and a value
// compared with itself learns nothing, so these checks can fail.

function aliasNotFalse(value: string | null) {
	// `ok` is `null` when `value` is `null`, and `null !== false`.
	const ok = value && value.length > 0;
	if (ok !== false) {
		return value ?? "";
	}
}

function looseNullable(value: string | null, other: string | null) {
	if (value == other) {
		return value ?? "";
	}
}

function comparedWithItself(value: string | null) {
	if (value === value) {
		return value ?? "";
	}
}
