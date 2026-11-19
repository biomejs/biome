/* should generate diagnostics */

// Ternary expressions: not reported
declare const a: string | null;
const r1 = a !== null ? a : 'default';

declare const b: string | undefined;
const r2 = b === undefined ? 'default' : b;

declare const c: string | null;
const r3 = c != null ? c : 'default';

declare const d: { a: string } | null;
const r4 = d ? d : { a: 'default' };
const r5 = !d ? { a: 'default' } : d;

// Other checks: reported
declare const s: string | null;
const r6 = s || 'x';

declare let assigned: string | null;
assigned ||= 'y';

declare let o: { a: number } | null;
if (!o) {
	o = { a: 1 };
}
