// should not generate diagnostics
// A function called where it is defined can leave the value `null` on one of
// its paths, or return before the check, and an `async` function called
// where it is defined runs apart from the code around it, so these checks
// can fail.

declare const reset: boolean;

function resetOnSomePaths(value: string | null) {
	if (value !== null) {
		(() => {
			if (reset) {
				value = null;
			}
		})();
		return value ?? "b";
	}
}

function returnsEarly(value: string | null) {
	(() => {
		if (value === null) {
			return;
		}
	})();
	return value ?? "b";
}

function assignedInAsync(value: string | null) {
	(async () => {
		value = "a";
	})();
	return value ?? "b";
}
