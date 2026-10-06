/* should generate diagnostics */
function shadowedBoolean(Boolean) { if (test) { return true; } else { return false; } }

function shadowedBooleanFlat(Boolean) { if (test) { return true; } return false; }

function commentInElse() { if (test) { return true; } else { return false; /* comment */ } }

function commentInIf() { if (test) { return true; /* comment */ } else { return false; } }

function commentInTest() { if (/* comment */ test) { return true; } return false; }

function commentBetween() {
	if (test) {
		return true;
	}

	// comment
	return false;
}

function commentAfterIf() {
	if (test) {
		return true;
	} // comment
	return false;
}

function outerShadowedBoolean() {
	const Boolean = 1;
	return function () { if (test) { return true; } return false; };
}
