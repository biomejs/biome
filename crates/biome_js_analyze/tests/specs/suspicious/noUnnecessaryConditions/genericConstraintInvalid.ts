// should generate diagnostics

// A generic parameter constrained to string literals is always truthy.
type Kind = "bar" | "baz";
function named<T extends Kind>(value: T) {
	if (value) {
	}
}

function inline<T extends "bar" | "baz">(value: T) {
	if (value) {
	}
}
