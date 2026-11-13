/* should generate diagnostics */
function returnsComparison() {
	return(0) < a - b;
}
function throwsComparison() {
	throw(0) < a - b;
}
function* yieldsComparison() {
	yield(0) < a - b;
}
function inComparison() {
	return a - b > (0)in object;
}

// The fix would change how these statements are parsed, so there is no fix.
foo()
0 < (a) - b;
foo()
0 < [a][0] - b;
foo()
0 < -a - b;
({ value: 1 }.value - other) > 0;
(function () {}.length - other) > 0;
const arrow = () => ({ value: 1 }.value - other) > 0;

// These keep the first token, so the fix is offered.
const result = ({ value: 1 }.value - other) > 0;
({ value: 1 }.value) - other > 0;
foo();
0 < a - b;
