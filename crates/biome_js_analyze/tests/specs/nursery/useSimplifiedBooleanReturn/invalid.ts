/* should generate diagnostics */
function asBoolean() { if (value as boolean) { return true; } return false; }

function typedParameter(value: boolean) { if (value) { return true; } return false; }

function satisfiesBoolean() { if (value satisfies boolean) { return true; } return false; }

function nonNull() { if (value!) { return true; } return false; }

function typeAssertion() { if (<boolean>value) { return true; } return false; }

function satisfiesNegated() { if (value satisfies boolean) { return false; } return true; }

function asConstReturn() {
	if (a) {
		return true as const;
	}

	return false;
}

function asBooleanReturn() {
	if (a) {
		return true as boolean;
	}

	return false;
}

function satisfiesReturn() {
	if (a) {
		return true satisfies boolean;
	}

	return false;
}

function asConstNegated() {
	if (a) {
		return false as const;
	}

	return true;
}

function nonNullNegated() {
	if (a) {
		return false!;
	}

	return true;
}
