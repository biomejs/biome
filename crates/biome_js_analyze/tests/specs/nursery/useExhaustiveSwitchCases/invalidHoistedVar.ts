// should generate diagnostics
// A `var` is `undefined` until its declarator runs, so where a path skips the
// declarator, it can be `undefined` as well as `null`, and the `switch` misses
// that case.

declare const enabled: boolean;

export function skipped() {
	if (enabled) {
		var pending = null;
	}
	switch (pending) {
		case null:
			return 1;
	}
}
