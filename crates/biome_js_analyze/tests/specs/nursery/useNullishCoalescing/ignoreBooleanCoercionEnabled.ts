/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;
declare const d: string;
declare const n: number | undefined;

// `||` inside `Boolean()`: not reported
const r1 = Boolean(a || b);

// Chained `||` inside `Boolean()`: not reported
const r2 = Boolean(a || b || c);

// Parenthesized argument: not reported
const r3 = Boolean((a || b));

// `||=` inside `Boolean()`: not reported
declare let assigned: string | null;
const r4 = Boolean(assigned ||= b);

// `Boolean()` nested in another call: not reported
declare function identity<T>(value: T): T;
const r5 = identity(Boolean(a || 'default'));

// `||` under `&&` inside `Boolean()`: not reported
const r6 = Boolean(d && (a || 'default'));

// Outside `Boolean()`
const r7 = a || 'default';
const r8 = n || 0;

declare let assignPlain: string | null;
assignPlain ||= 'default';

// Inside another call
const r9 = identity(a || 'default');

// `Boolean` shadowed by a parameter
function shadowedByParameter(Boolean: (value: unknown) => boolean) {
	return Boolean(a || 'default');
}

// `Boolean` shadowed by a local variable
function shadowedByLocal() {
	const Boolean = (value: unknown) => !!value;
	return Boolean(a || 'default');
}

// `||` in a ternary branch or the last expression of a sequence: not reported
declare const flag: boolean;
const r10 = Boolean(flag ? a || b : 'default');
const r11 = Boolean(flag ? 'default' : a || b);
const r12 = Boolean((identity(flag), a || b));

// `||` as an operand of `??` inside `Boolean()`: not reported
const r13 = Boolean(a ?? (b || c));
const r14 = Boolean((a || b) ?? c);

// Negated inside `Boolean()`
const r15 = Boolean(!(a || 'default'));

// Earlier expression of a sequence
const r16 = Boolean((a || 'default', flag));

// Calls other than the global `Boolean()`, or a value that isn't the argument
const r17 = new Boolean(a || b);
const r18 = String(a || b);
const r19 = Boolean(1 + (a || b));
const r20 = Boolean(() => a || b);

// Comparison ternary inside `Boolean()`
const r21 = Boolean(a != null ? a : b);
