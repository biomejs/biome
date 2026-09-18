// =====================
// While statements
// =====================

// Simple while statement
while (a) {}

// Nested parentheses (inner parens not affected)
while ((a || b) && c) {}

// Function call in condition
while (foo()) {}

// Boundary: 78 chars + 2 spaces = 80 chars (fits)
while (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {}

// Boundary: 79 chars + 2 spaces = 81 chars (breaks)
while (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa) {}

// =====================
// Do-while statements
// =====================

// Simple do-while statement
do {} while (a);

// Nested parentheses (inner parens not affected)
do {} while ((a || b) && c);

// Function call in condition
do {} while (foo());

// Boundary: 78 chars input + 2 spaces = 80 chars (fits on line)
do {} while (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa);

// Boundary: 79 chars input + 2 spaces = 81 chars (breaks to multiple lines)
do {} while (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa);

// =====================
// For statements
// =====================

// Simple for statement
for (a; b; c) {}

// With variable declaration
for (let a = 0; a < b; a++) {}

// Nested parentheses in condition (inner parens not affected)
for (let a = 0; (a || b) && c; a++) {}

// Function call in condition
for (let a = 0; foo(); a++) {}

// Empty parts (no spaces when empty)
for (;;) {}

// Boundary: 78 chars (fits at 80 with spaces)
for (let a = 0; a < bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb; a++) {}

// Boundary: 79 chars (breaks at 81 with spaces)
for (let a = 0; a < bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb; a++) {}

// =====================
// For-in statements
// =====================

// Simple for-in statement
for (a in b) {}

// For-in with body
for (a in b) {
	foo();
}

// For-in with let declaration
for (let a in b) {}

// For-in with const declaration
for (const a in b) {}

// For-in with var declaration
for (var a in b) {}

// For-in with member expression
for (a.b in c) {}

// For-in with bracket notation
for (a[b] in c) {}

// For-in with destructuring array
for (const [a, b] in c) {}

// For-in with destructuring object
for (const { a, b } in c) {}

// Expression in RHS (parentheses removed as unnecessary)
for (a in (b || c)) {}

// Function call in RHS
for (a in foo()) {}

// For-in with nested body
for (a in b) {
	for (c in d) {
		foo();
	}
}

// =====================
// For-of statements
// =====================

// Simple for-of statement
for (a of b) {}

// For-of with body
for (a of b) {
	foo();
}

// For-of with let declaration
for (let a of b) {}

// For-of with const declaration
for (const a of b) {}

// For-of with var declaration
for (var a of b) {}

// For-of with destructuring array
for (const [a, b] of c) {}

// For-of with destructuring object
for (const { a, b } of c) {}

// For-of with member expression
for (a.b of c) {}

// For-of with bracket notation
for (a[b] of c) {}

// Expression in RHS (parentheses removed as unnecessary)
for (a of (b || c)) {}

// Function call in RHS
for (a of foo()) {}

// Async for-of statement
async function foo() {
	for await (a of b) {}
}

// For-of with nested body
for (a of b) {
	for (c of d) {
		foo();
	}
}
