/* should generate diagnostics */
function returnsUndefined(): undefined {
	return undefined;
}

function returnsVoid(): void {
	return undefined;
}

function returnsParenthesizedUndefined(): ((undefined)) {
	return undefined;
}

function returnsParenthesizedVoid(): (void) {
	return undefined;
}

function outer(): string {
	function inner() {
		return undefined;
	}

	return "";
}
