/* should not generate diagnostics */

// Correct strict (in)equality.
if (foo !== bar) {
}

if (foo === bar) {
}

// Negating the whole comparison (grouped) is intentional.
if (!(foo === bar)) {
}

// Negation on the right operand is a deliberate comparison, not this bug.
const a = foo === !bar;

// Loose equality is out of scope — the rule targets `===` / `!==`.
if (!foo == bar) {
}
