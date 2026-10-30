/* should generate diagnostics */
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
