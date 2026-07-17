// =====================
// If statements
// =====================

// Simple if statement
if (a) {
}

// If statement with else
if (a) {
} else {
}

// If-else-if chain
if (a) {
} else if (b) {
} else {
}

// Complex condition
if (a && b) {
}

// Nested condition
if ((a || b) && c) {
}

// Comparison
if (a === b) {
}

// Function call in condition
if (foo()) {
}

// Multiline condition (fits on one line)
if (a && b && c) {
}

// Boundary: 80 chars with delimiterSpacing (fits)
if (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {
}

// Boundary: 81 chars with delimiterSpacing (breaks)
if (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {
}

// =====================
// Switch statements
// =====================

// Simple switch statement
switch (a) {
	case 1:
		break;
}

// Switch with expression
switch (a + b) {
	case 1:
		break;
}

// Switch with function call
switch (foo()) {
	case 1:
		break;
}

// Nested parentheses
switch ((a || b) && c) {
	case true:
		break;
}

// Comparison
switch (a === b) {
	case true:
		break;
}

// Multiline expression (fits on one line)
switch (a + b + c) {
	case 1:
		break;
}

// Boundary: 80 chars with delimiterSpacing (fits)
switch (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {}

// Boundary: 81 chars with delimiterSpacing (breaks)
switch (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {}
