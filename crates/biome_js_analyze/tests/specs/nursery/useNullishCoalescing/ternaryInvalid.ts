/* should generate diagnostics */

// `!== null`
declare const a: string | null;
const r1 = a !== null ? a : 'default';

// `!== undefined`
declare const b: string | undefined;
const r2 = b !== undefined ? b : 'fallback';

// `!= null`
declare const c: string | null;
const r3 = c != null ? c : 'default';

// `=== null`
declare const d: string | null;
const r4 = d === null ? 'default' : d;

// `=== undefined`
declare const e: string | undefined;
const r5 = e === undefined ? 'fallback' : e;

// `== null`
declare const f: string | null;
const r6 = f == null ? 'default' : f;

// `!== null && !== undefined`
declare const g: string | null | undefined;
const r7 = g !== null && g !== undefined ? g : 'default';

// `=== null || === undefined`
declare const h: string | null | undefined;
const r8 = h === null || h === undefined ? 'default' : h;

// `null` on the left
declare const i: string | null;
const r9 = null !== i ? i : 'default';

// `undefined` on the left
declare const j: string | undefined;
const r10 = undefined !== j ? j : 'fallback';

// Member access
declare const obj: { prop: string | null };
const r11 = obj.prop !== null ? obj.prop : 'default';

// `!= undefined`
declare const l: string | undefined;
const r12 = l != undefined ? l : 'default';

// `=== null` with `null` on the left
declare const n: string | null;
const r13 = null === n ? 'default' : n;

// Conditional fallback is parenthesized
declare const p: string | null;
const r14 = p !== null ? p : p ? 'a' : 'b';

// `||` fallback is parenthesized
declare const q: string | null;
const r15 = q !== null ? q : q || 'default';

// `&&` fallback is parenthesized
declare const s: string | null;
const r16 = s !== null ? s : s && 'value';

// Call in the subject: no fix
declare function foo(): string | null;
const r17 = foo() !== null ? foo() : 'default';

// `new` in the subject: no fix
declare class Bar { value: string | null }
const r18 = new Bar().value !== null ? new Bar().value : 'default';

// `undefined` type
const undef: undefined = undefined;
const r19 = undef !== undefined ? undef : 'fallback';

// `number | undefined`
declare const maybeNum: number | undefined;
const r20 = maybeNum !== undefined ? maybeNum : 42;

// `!= null` on a union with both `null` and `undefined`
declare const tripleUnion: string | null | undefined;
const r21 = tripleUnion != null ? tripleUnion : 'default';

// Optional property
interface Config { timeout?: number; }
declare const config: Config;
const r22 = config.timeout !== undefined ? config.timeout : 3000;

// Array element
declare const arr: (number | null)[];
const r23 = arr[0] !== null ? arr[0] : 0;

// Function return
function getVal(x: string | null): string {
  return x !== null ? x : 'default';
}

// Parenthesized ternary
declare const paren: string | null;
const r24 = (paren !== null ? paren : 'default').toUpperCase();

// Strict check of the only nullish variant

// `!== null` on a union with `null`
declare const ss5: string | null;
const r25 = ss5 !== null ? ss5 : 'default';

// `!== undefined` on a union with `undefined`
declare const ss6: string | undefined;
const r26 = ss6 !== undefined ? ss6 : 'default';

// Compound check of both nullish variants

// `null` then `undefined`
declare const cp2: string | null | undefined;
const r27 = cp2 !== null && cp2 !== undefined ? cp2 : 'default';

// `undefined` then `null`
declare const cp3: string | null | undefined;
const r28 = cp3 !== undefined && cp3 !== null ? cp3 : 'default';
