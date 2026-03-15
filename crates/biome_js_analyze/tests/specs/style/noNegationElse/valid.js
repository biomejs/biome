/* should not generate diagnostics */

if (true) {
	consequent;
} else {
	alternate;
}

if (a == b) {
	consequent;
} else {
	alternate;
}

if (a === b) {
	consequent;
} else {
	alternate;
}

// Single branch
if (!true) {
	consequent;
}

if (a != b) {
	consequent;
}

if (a !== b) {
	consequent;
}

// Ternary
true ? consequent : alternate;
a == b ? consequent : alternate;
a === b ? consequent : alternate;

// https://github.com/rome/tools/issues/2999
if (!/^NCT/.test(input)) {
	messages.push("NCT Number must start with NCT");
} else if (!/^NCT\d{8}$/.test(input)) {
	messages.push("NCT Number must have exactly 8 digits after NCT");
}

// https://github.com/rome/tools/issues/3141
function f() {
	return !!specs.variables ? specs.variables(props) : {};
}

if (+5) { } else { }
if (-5) { } else { }
if (void f()) { } else { }
