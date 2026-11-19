// Tests for ignoreBooleanCoercion: false
// `||` and `||=` inside a Boolean() call SHOULD still generate diagnostics.

declare const a: string | null;
declare const b: string;
declare const c: string;

// || directly inside Boolean()
const r1 = Boolean(a || b);

// chained || inside Boolean()
const r2 = Boolean(a || b || c);

// ||= inside Boolean()
declare let assigned: string | null;
const r3 = Boolean(assigned ||= b);
