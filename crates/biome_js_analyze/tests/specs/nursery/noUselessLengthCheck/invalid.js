/* should generate diagnostics */

// `every()` returns true for [], so the emptiness check is redundant.
const a = array.length === 0 || array.every(Boolean);
const b = array.length < 1 || array.every(Boolean);

// `some()` returns false for [], so the non-emptiness check is redundant.
const c = array.length !== 0 && array.some(Boolean);
const d = array.length > 0 && array.some(Boolean);
const e = array.length >= 1 && array.some(Boolean);

// Length check on the right operand.
const f = array.every(Boolean) || array.length === 0;

// Member-chain receiver.
const g = foo.bar.length === 0 || foo.bar.every(Boolean);
