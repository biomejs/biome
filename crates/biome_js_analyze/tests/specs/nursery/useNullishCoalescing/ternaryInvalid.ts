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

// `undefined` type
const undef: undefined = undefined;
const r17 = undef !== undefined ? undef : 'fallback';

// `number | undefined`
declare const maybeNum: number | undefined;
const r18 = maybeNum !== undefined ? maybeNum : 42;

// `!= null` on a union with both `null` and `undefined`
declare const tripleUnion: string | null | undefined;
const r19 = tripleUnion != null ? tripleUnion : 'default';

// Optional property
interface Config { timeout?: number; }
declare const config: Config;
const r20 = config.timeout !== undefined ? config.timeout : 3000;

// Array element
declare const arr: (number | null)[];
const r21 = arr[0] !== null ? arr[0] : 0;

// Function return
function getVal(x: string | null): string {
  return x !== null ? x : 'default';
}

// Parenthesized ternary
declare const paren: string | null;
const r22 = (paren !== null ? paren : 'default').toUpperCase();

// Strict check of the only nullish variant

// `!== null` on a union with `null`
declare const ss5: string | null;
const r23 = ss5 !== null ? ss5 : 'default';

// `!== undefined` on a union with `undefined`
declare const ss6: string | undefined;
const r24 = ss6 !== undefined ? ss6 : 'default';

// Compound check of both nullish variants

// `null` then `undefined`
declare const cp2: string | null | undefined;
const r25 = cp2 !== null && cp2 !== undefined ? cp2 : 'default';

// `undefined` then `null`
declare const cp3: string | null | undefined;
const r26 = cp3 !== undefined && cp3 !== null ? cp3 : 'default';

// Parentheses around the test, a literal, or a compound side
declare const t1: string | null;
const r27 = (t1 !== null) ? t1 : 'default';
const r28 = t1 != (null) ? t1 : 'default';
const r29 = ((t1 === null)) ? 'default' : t1;

declare const t2: string | null | undefined;
const r30 = (t2 !== null) && (t2 !== undefined) ? t2 : 'default';
const r31 = (t2 === null || t2 === undefined) ? 'default' : t2;

// Compound check mixing strict and loose comparisons
declare const mixed: string | null | undefined;
const r32 = mixed != undefined && mixed !== null ? mixed : 'default';
const r33 = mixed !== undefined && mixed != null ? mixed : 'default';
const r34 = mixed != undefined && mixed != null ? mixed : 'default';
const r35 = mixed == undefined || mixed === null ? 'default' : mixed;
const r36 = mixed == undefined || mixed == null ? 'default' : mixed;

// Optional chaining and bracket notation in the test
declare const nested: { n?: { a?: string } };
const r37 = nested.n?.a !== undefined ? nested.n.a : 'default';
const r38 = nested.n?.a != null ? nested?.n.a : 'default';

declare const maybeObject: { a: string; b: string } | null;
const r39 = maybeObject?.['a'] != null ? maybeObject.a : 'default';
const r40 = maybeObject?.a != null ? maybeObject['a'] : 'default';

declare const key: 'a' | 'b';
const r41 = maybeObject?.[key] != null ? maybeObject[key] : 'default';

// Compound check with optional chaining on one side only: no fix
declare const deep: { n?: { a?: string | null } };
const r42 = deep.n?.a !== undefined && deep.n.a !== null ? deep.n.a : 'default';
const r43 = deep.n?.a !== null && deep.n.a !== undefined ? deep.n.a : 'default';

// Arrow function fallback
declare const arrowSubject: string | null;
const r44 = arrowSubject != null ? arrowSubject : () => 'y';
const r45 = arrowSubject != null ? arrowSubject : async () => 'y';

// Type wrappers across the test and the branch: no fix
declare const raw: unknown;
const r46 = raw != null ? (raw as string) : '';

declare const castTest: string | null;
const r47 = (castTest as string | null) !== null ? castTest : 'default';

declare const bang: string | null;
const r48 = bang != null ? bang! : 'default';

declare const sat: string | null;
const r49 = sat != null ? (sat satisfies string | null) : 'default';

declare const angle: unknown;
const r50 = angle != null ? <string>angle : '';

declare const generic: (<T>() => T) | null;
const r51 = generic != null ? generic<string> : null;

// Nested type assertions: no fix
type Nested = { v: string | null };
declare const wide: { v: string | number | null };
const r55 = wide.v != null ? (wide as Nested).v : '';

declare const anyObject: any;
const r56 = (anyObject as any).v != null ? anyObject.v : 'default';

// Line breaks and comments around the present branch
export function returned(value: string | null) {
	return value != null ?
		value :
		'default';
}

export function returnedWithLineComment(value: string | null) {
	return value != null ? // note
		value : 'default';
}

export function returnedWithBlockComment(value: string | null) {
	return value != null ? /* note */
		value : 'default';
}

export function returnedInverted(value: string | null) {
	return value == null ?
		'default' :
		value;
}

export function thrown(value: Error | null) {
	throw value != null ?
		value :
		new Error('default');
}

export function formatted(value: string | null) {
	const result = value != null
		? value
		: 'default';
	return result;
}

// Nested ternary
declare const nestedA: string | null;
declare const nestedB: string | null;
const r52 = nestedA != null ? nestedA : nestedB != null ? nestedB : 'default';

// `this`, private fields, and `super`
declare class Base {
	x: string | null;
}
class Refs extends Base {
	#p: string | null = null;
	items: (string | null)[] = [];

	thisProperty() {
		return this.x != null ? this.x : 'default';
	}

	privateField() {
		return this.#p != null ? this.#p : 'default';
	}

	superProperty() {
		return super.x != null ? super.x : 'default';
	}

	numericKey() {
		return this.items[0] != null ? this.items['0'] : 'default';
	}
}

// Template literal key and string key
declare const tpl: { b: string | null };
const r53 = tpl[`b`] != null ? tpl.b : 'default';
const r54 = tpl.b != null ? tpl['b'] : 'default';

// A fix starting with `(` would continue the previous statement: no fix
declare const holder: { b?: string | null };
declare function log(): () => unknown;
log()
null != (holder.b) ? (holder.b) : 'default'

// Previous statement ends with `;`
log();
null != (holder.b) ? (holder.b) : 'default'
