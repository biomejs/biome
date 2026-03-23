// Strict !== null
declare const a: string | null;
const r1 = a !== null ? a : 'default';

// Strict !== undefined
declare const b: string | undefined;
const r2 = b !== undefined ? b : 'fallback';

// Loose != null
declare const c: string | null;
const r3 = c != null ? c : 'default';

// Inverted === null
declare const d: string | null;
const r4 = d === null ? 'default' : d;

// Inverted === undefined
declare const e: string | undefined;
const r5 = e === undefined ? 'fallback' : e;

// Inverted == null
declare const f: string | null;
const r6 = f == null ? 'default' : f;

// Compound !== null && !== undefined
declare const g: string | null | undefined;
const r7 = g !== null && g !== undefined ? g : 'default';

// Compound === null || === undefined
declare const h: string | null | undefined;
const r8 = h === null || h === undefined ? 'default' : h;

// Null on left side
declare const i: string | null;
const r9 = null !== i ? i : 'default';

// Undefined on left side
declare const j: string | undefined;
const r10 = undefined !== j ? j : 'fallback';

// Member access
declare const obj: { prop: string | null };
const r11 = obj.prop !== null ? obj.prop : 'default';

// Loose != undefined
declare const l: string | undefined;
const r12 = l != undefined ? l : 'default';

// Inverted, null on left
declare const n: string | null;
const r13 = null === n ? 'default' : n;

// Strict single with null | undefined (diagnostic only, no fix)
declare const nu: string | null | undefined;
const r17 = nu !== null ? nu : 'default';

// Fallback is conditional (needs parens)
declare const p: string | null;
const r14 = p !== null ? p : p ? 'a' : 'b';

// Fallback is logical OR (needs parens, ?? cannot mix with ||)
declare const q: string | null;
const r15 = q !== null ? q : q || 'default';

// Fallback is logical AND (needs parens, ?? cannot mix with &&)
declare const s: string | null;
const r16 = s !== null ? s : s && 'value';

// Call expression in subject (diagnostic only, no fix: side-effect safety)
declare function foo(): string | null;
const r18 = foo() !== null ? foo() : 'default';

// New expression in subject (diagnostic only, no fix: side-effect safety)
declare class Bar { value: string | null }
const r25 = new Bar().value !== null ? new Bar().value : 'default';

// Undefined literal type
const undef: undefined = undefined;
const r19 = undef !== undefined ? undef : 'fallback';

// Number | undefined (non-string union)
declare const maybeNum: number | undefined;
const r20 = maybeNum !== undefined ? maybeNum : 42;

// Loose != null with triple union (covers both null and undefined)
declare const tripleUnion: string | null | undefined;
const r21 = tripleUnion != null ? tripleUnion : 'default';

// Optional property (a?: string creates string | undefined)
interface Config { timeout?: number; }
declare const config: Config;
const r22 = config.timeout !== undefined ? config.timeout : 3000;

// Array element access
declare const arr: (number | null)[];
const r23 = arr[0] !== null ? arr[0] : 0;

// Function return context
function getVal(x: string | null): string {
  return x !== null ? x : 'default';
}

// Nested in parentheses
declare const paren: string | null;
const r24 = (paren !== null ? paren : 'default').toUpperCase();

// --- StrictSingle fixability: opposite nullish variant in type ---

// Checks !== null but type has undefined (not null): no fix
declare const ss1: string | undefined;
const r26 = ss1 !== null ? ss1 : 'default';

// Checks !== undefined but type has null (not undefined): no fix
declare const ss2: string | null;
const r27 = ss2 !== undefined ? ss2 : 'default';

// Inverted: checks === null but type has undefined: no fix
declare const ss3: string | undefined;
const r28 = ss3 === null ? 'default' : ss3;

// Inverted: checks === undefined but type has null: no fix
declare const ss4: string | null;
const r29 = ss4 === undefined ? 'default' : ss4;

// Checks !== null, type has only null: fix IS safe
declare const ss5: string | null;
const r30 = ss5 !== null ? ss5 : 'default';

// Checks !== undefined, type has only undefined: fix IS safe
declare const ss6: string | undefined;
const r31 = ss6 !== undefined ? ss6 : 'default';

// Checks !== null, type has void (acts as undefined): no fix
declare const ss7: string | void;
const r32 = ss7 !== null ? ss7 : 'default';

// --- Compound fixability: duplicate vs complementary literals ---

// Both sides check null (not complementary): no fix
declare const cp1: string | null | undefined;
const r33 = cp1 !== null && cp1 !== null ? cp1 : 'default';

// Complementary: one null, one undefined: fix IS safe
declare const cp2: string | null | undefined;
const r34 = cp2 !== null && cp2 !== undefined ? cp2 : 'default';

// Complementary reversed order: fix IS safe
declare const cp3: string | null | undefined;
const r35 = cp3 !== undefined && cp3 !== null ? cp3 : 'default';
