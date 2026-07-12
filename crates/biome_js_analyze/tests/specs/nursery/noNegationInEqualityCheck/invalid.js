/* should generate diagnostics */

// Negated left operand of a strict equality check.
if (!foo === bar) {
}

// Negated left operand of a strict inequality check.
if (!foo !== bar) {
}

// In an assignment / expression position.
const a = !x === y;
