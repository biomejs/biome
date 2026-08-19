// Tests for ignoreBooleanCoercion: true
// `||` and `||=` inside a Boolean() call should NOT generate diagnostics.

declare const a: string | null;
declare const b: string;
declare const c: string;

// || directly inside Boolean()
const r1 = Boolean(a || b);

// chained || inside Boolean()
const r2 = Boolean(a || b || c);

// || inside parenthesized argument
const r3 = Boolean((a || b));

// ||= inside Boolean()
declare let assigned: string | null;
const r4 = Boolean(assigned ||= b);

// || nested inside Boolean() within another call: still suppressed because the
// value still flows into the Boolean() coercion.
declare function identity<T>(value: T): T;
const r4b = identity(Boolean(a || 'default'));

// || mixed with && inside Boolean(): the whole argument is boolean-coerced,
// so the walk steps through the && parent and suppresses the ||.
declare const d: string;
const r4c = Boolean(d && (a || 'default'));

// These SHOULD still generate diagnostics (not inside Boolean()).
const r5 = a || 'default';

declare const n: number | undefined;
const r6 = n || 0;

declare let assignPlain: string | null;
assignPlain ||= 'default';

// Inside a non-Boolean call: SHOULD generate a diagnostic.
const r7 = identity(a || 'default');
