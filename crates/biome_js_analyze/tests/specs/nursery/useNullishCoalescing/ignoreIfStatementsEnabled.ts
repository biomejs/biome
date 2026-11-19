/* should generate diagnostics */

declare let a: string | null;

// `if` statements: not reported
if (!a) {
	a = 'default';
}

if (a == null) {
	a = 'fallback';
}

// `||`
declare const s: string | null;
const r1 = s || 'x';

// `||=`
declare let b: string | null;
b ||= 'y';

// Truthiness ternary: reported
declare const o: { a: number } | null;
const r2 = o ? o : { a: 1 };
