/* should generate diagnostics */

declare const fallback: string;

// Primitive unions: no fix
declare const maybeString: string | null | undefined;
const r1 = maybeString ? maybeString : fallback;
const r2 = !maybeString ? fallback : maybeString;

declare const maybeNumber: number | undefined;
const r3 = maybeNumber ? maybeNumber : 0;

declare const maybeBoolean: boolean | null;
const r4 = !maybeBoolean ? false : maybeBoolean;

declare const maybeLiteral: 'a' | `b` | undefined;
const r5 = maybeLiteral ? maybeLiteral : fallback;

// `any` and `unknown`: no fix
declare const anyValue: any;
const r6 = anyValue ? anyValue : fallback;

declare const unknownValue: unknown;
const r7 = !unknownValue ? fallback : unknownValue;

// Object union: safe fix
declare const maybeObject: { a: string } | null | undefined;
const r8 = maybeObject ? maybeObject : { a: fallback };
const r9 = !maybeObject ? { a: fallback } : maybeObject;

// Parentheses around the test
const r10 = (maybeObject) ? maybeObject : { a: fallback };
const r11 = !(maybeObject) ? { a: fallback } : maybeObject;

// Property access
declare const holder: { n: { a: string } | undefined };
const r12 = holder.n ? holder.n : { a: fallback };
const r13 = !holder.n ? { a: fallback } : holder.n;

// Optional chaining in the test
declare const nested: { n?: { a?: { b: string } } };
const r14 = nested.n?.a ? nested.n.a : { b: fallback };
const r15 = nested?.n?.a ? nested.n?.a : { b: fallback };
const r16 = nested.n?.a ? (nested?.n).a : { b: fallback };

// Conditional fallback
declare const other: { a: string } | undefined;
const r17 = maybeObject ? maybeObject : other ? other : { a: fallback };

// Arrow function fallback
const r18 = other ? other : () => 'y';

// A type that a `0`, `''`, or `false` value can have: no fix
declare const numberObject: Number | null;
const r19 = numberObject ? numberObject : 1;

// A type that no `0`, `''`, or `false` value can have: safe fix
declare const named: { a: string } | null;
const r20 = named ? named : { a: '' };

// A type parameter is checked through its constraint
export function chainedGeneric<T extends object, U extends T>(value: U | null, fallbackValue: U) {
	return value ? value : fallbackValue;
}

// Type wrappers across the test and the branch: no fix
declare const shape: { v: { a: string } | null };
const r21 = (shape as { v: { a: string } | null }).v ? shape.v : { a: fallback };
const r22 = shape.v ? (shape.v as { a: string }) : { a: fallback };
const r23 = shape.v ? shape.v! : { a: fallback };
const r24 = !shape.v ? { a: fallback } : (shape.v satisfies { a: string } | null);
const r25 = shape.v ? <{ a: string }>shape.v : { a: fallback };
