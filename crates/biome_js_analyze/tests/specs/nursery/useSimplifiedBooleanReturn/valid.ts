/* should not generate diagnostics */
function nonNullIdentifier() {
	if (a) {
		return t!;
	}

	return false;
}
