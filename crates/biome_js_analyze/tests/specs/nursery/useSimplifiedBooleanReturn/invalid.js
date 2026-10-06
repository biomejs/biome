/* should generate diagnostics */
function ifElseBlocks() { if (test) { return true; } else { return false; } }

function ifElseStatements() { if (test) return true; else return false; }

function ifElseComparison() { if (value > 0) { return true; } else { return false; } }

function ifElseNegated() { if (items.length) { return false; } else { return true; } }

function flat() { if (test) { return true; } return false; }

function flatNegated() { if (test) { return false; } return true; }

function flatComparison() { if (value > 0) { return true; } return false; }

function flatNegatedComparison() { if (value > 0) { return false; } return true; }

function flatStatement() { if (test) return true; return false; }

function booleanCall() { if (Boolean(value)) { return true; } return false; }

function globalThisBooleanCall() { if (globalThis.Boolean(value)) { return true; } return false; }

function logicalOfComparisons() { if (a === b && c === d) { return true; } return false; }

function logicalOfValues() { if (a && b) { return true; } return false; }

function longLogical() { if (a === b && c !== d || e in f && g instanceof h) { return true; } return false; }

function partialLogical() { if (a === b && c || d === e) { return true; } return false; }

function arrayIsArray() { if (Array.isArray(value)) { return true; } return false; }

function shadowedArray(Array) { if (Array.isArray(value)) { return true; } return false; }

function optionalChain() { if (object?.property) { return true; } return false; }

function sequence() { if (a, b) { return true; } return false; }

function negation() { if (!test) { return true; } return false; }

function deletion() { if (delete object.key) { return true; } return false; }

function typeofValue() { if (typeof value) { return true; } return false; }

function logicalWithLiteral() { if (a === b || false) { return true; } return false; }

function assignment() { if (a = b) { return false; } return true; }

function parenthesizedReturn() { if (test) { return (true); } return (false); }

function nestedBlocks() { if (test) { { return true; } } return false; }

function emptyStatements() { if (test) { ; return true; ; } return false; }

function shadowedBooleanNegated(Boolean) { if (test) { return false; } return true; }

function shadowedBooleanComparison(Boolean) { if (value > 0) { return true; } return false; }

function differentEarlyReturn() { if (a) { return false; } if (b) { return true; } return false; }

function elseIf() {
	if (a) {
		return 1;
	} else if (b) {
		return true;
	} else {
		return false;
	}
}

function multiline() {
	if (test({
		multiline: true,
	})) {
		return true;
	}

	return false;
}

function leadingComment() {
	// leading comment
	if (test) {
		return true;
	}
	return false; // trailing comment
}

function noSemicolon() {
	if (test) {
		return true
	}
	return false
}

function inSwitch(value) {
	switch (value) {
		case 1:
			if (test) return true;
			return false;
	}
}

const arrow = () => { if (test) { return true; } return false; };

function globalThisArrayIsArray() { if (globalThis.Array.isArray(value)) { return true; } return false; }

function optionalArrayIsArray() { if (Array?.isArray(value)) { return true; } return false; }

function parenthesizedSequence() { if ((a, b)) { return true; } return false; }

function negatedArrow() { if (() => {}) { return false; } return true; }

function negatedAsyncArrow() { if (async () => {}) { return false; } return true; }
