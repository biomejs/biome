// A `let` declared without an initializer takes the types of the values that
// the code assigns to it, as in TypeScript, so the left operand of each `||`
// below may be nullish.

declare const enabled: boolean;

function written() {
	let label;
	if (enabled) label = "on";
	return label || "off";
}

function initializedWithNull() {
	let label = null;
	if (enabled) label = "on";
	return label || "off";
}
