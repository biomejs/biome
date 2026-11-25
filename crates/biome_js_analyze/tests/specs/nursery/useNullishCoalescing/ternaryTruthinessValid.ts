/* should not generate diagnostics */

declare const fallback: string;

// Non-nullish value
declare const definite: string;
const r1 = definite ? definite : fallback;
const r2 = !definite ? fallback : definite;

declare const neverValue: never;
const r3 = neverValue ? neverValue : fallback;

// Present branch with a different value
declare const maybe: string | null;
declare const other: string;
const r4 = maybe ? other : fallback;
const r5 = !maybe ? fallback : other;

// Tested value in the falsy branch
const r6 = maybe ? fallback : maybe;
const r7 = !maybe ? maybe : fallback;

// Calls and other expressions
declare function read(): string | null;
const r8 = read() ? read() : fallback;
const r9 = !!maybe ? maybe : fallback;
const r10 = maybe! ? maybe : fallback;

// A type wrapper around the tested value is not a variable or a property
declare const wrappedObject: { a: string } | null;
const r11 = (wrappedObject as { a: string } | null) ? wrappedObject : { a: fallback };
