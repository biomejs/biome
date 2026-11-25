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

// Built-in object class union: safe fix
declare const maybeObject: Date | null | undefined;
const r8 = maybeObject ? maybeObject : new Date();
const r9 = !maybeObject ? new Date() : maybeObject;

// Parentheses around the test
const r10 = (maybeObject) ? maybeObject : new Date();
const r11 = !(maybeObject) ? new Date() : maybeObject;

// Property access
declare const holder: { n: Date | undefined };
const r12 = holder.n ? holder.n : new Date();
const r13 = !holder.n ? new Date() : holder.n;

// Optional chaining in the test
declare const nested: { n?: { a?: Date } };
const r14 = nested.n?.a ? nested.n.a : new Date();
const r15 = nested?.n?.a ? nested.n?.a : new Date();
const r16 = nested.n?.a ? (nested?.n).a : new Date();

// Conditional fallback
declare const other: Date | undefined;
const r17 = maybeObject ? maybeObject : other ? other : new Date();

// Arrow function fallback
const r18 = other ? other : () => 'y';

// A type that a `0`, `''`, or `false` value can have: no fix
declare const numberObject: Number | null;
const r19 = numberObject ? numberObject : 1;

// An object shape: no fix, because `''` can be assigned to a shape whose members all exist on `String`
declare const named: { a: string } | null;
const r20 = named ? named : { a: '' };

// A type parameter is checked through its constraint
export function chainedGeneric<T extends object, U extends T>(value: U | null, fallbackValue: U) {
	return value ? value : fallbackValue;
}

// Type wrappers across the test and the branch: no fix
declare const shape: { v: Date | null };
const r21 = (shape as { v: Date | null }).v ? shape.v : new Date();
const r22 = shape.v ? (shape.v as Date) : new Date();
const r23 = shape.v ? shape.v! : new Date();
const r24 = !shape.v ? new Date() : (shape.v satisfies Date | null);
const r25 = shape.v ? <Date>shape.v : new Date();
