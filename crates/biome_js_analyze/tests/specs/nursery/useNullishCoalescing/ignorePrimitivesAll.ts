/* should generate diagnostics */

declare const s: string | null;
declare const n: number | null;
declare const b: boolean | null;
declare const big: bigint | null;
declare const fallbackS: string;
declare const fallbackN: number;

// Primitive operands: not reported
const r1 = s || fallbackS;
const r2 = n || fallbackN;
const r3 = b || false;
const r4 = big || 0n;

// `||=` on a primitive: not reported
declare let assigned: string | null;
assigned ||= fallbackS;

// Literal unions: not reported
declare const litStr: 'a' | 'b' | null;
declare const litNum: 0 | 1 | null;
declare const litBool: true | null;
declare const litBig: 1n | 2n | null;
const r5 = litStr || 'c';
const r6 = litNum || 2;
const r7 = litBool || false;
const r8 = litBig || 3n;

// Object union: reported
declare const obj: Date | null;
declare const fallbackObj: Date;
const r9 = obj || fallbackObj;

// `any` and `unknown`: not reported
declare const anyValue: any;
declare const unknownValue: unknown;
const r10 = anyValue || fallbackS;
const r11 = unknownValue || fallbackS;

// Truthiness ternary and `if` on a primitive: not reported
const r12 = s ? s : fallbackS;
declare let assignedInIf: string | null;
if (!assignedInIf) {
	assignedInIf = fallbackS;
}

// Comparisons with `null`: reported
const r13 = s != null ? s : fallbackS;
if (assignedInIf == null) {
	assignedInIf = fallbackS;
}

// `||=` on `any`: not reported
declare let anyAssigned: any;
anyAssigned ||= 1;
