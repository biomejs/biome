/* should generate diagnostics */

declare const a: object | null;
declare const b: object | undefined;

// `||` with comments
const x = a /* inline comment */ || {};
const y = a || /* inline comment */ {};
const z = a /* before */ || /* after */ {};

// Line comments around the operator
const w = b
  // line comment before
  || {};

const v = b ||
  // line comment after
  {};

// `||=` with comments
declare let assigned: object | null;

assigned /* before */ ||= {};
assigned ||= /* after */ {};
assigned /* before */ ||= /* after */ {};

// Ternary with comments
declare const nonNull: string | null;
const t1 = nonNull !== null ? /* non-null value */ nonNull : /* fallback */ 'default';

declare const checked: string | null;
const t2 = /* check */ checked === null ? 'fallback' : checked;

declare const negated: string | null;
const t3 = negated === null ? /* fallback */ 'default' : /* value */ negated;

// Ternary with comments in dropped parts: no fix
declare const dropped: string | null;
const t4 = dropped !== null ? dropped /* after value */ : 'y';
const t5 = dropped /* in test */ !== null ? dropped : 'y';
const t6 = dropped !== null ? dropped
	// before colon
	: 'y';

// Comments in the test, around `?`, or inside the kept branch: no fix
declare const dropped1: string | null;
declare const dropped2: string | null | undefined;
const t7 = dropped1 == null /* c */ ? 'y' : dropped1;
const t8 = dropped1 == null ? 'y' /* c */ : dropped1;
const t9 = dropped1 != null ? (/* c */ dropped1) : 'y';
const t10 = dropped2 !== null /* c */ && dropped2 !== undefined ? dropped2 : 'y';
const t11 = dropped1 != null
	// c
	? dropped1 : 'y';

// Ternary with a trailing line comment and no semicolon
declare const trailing: string | null;
const t12 = trailing !== null ? trailing : 'y' // keep me

// Comments before the test and before the present branch
declare const lead: string | null;
const t13 = /* lead */ lead !== null ? /* present */ lead : 'd';

// `if` statement with comments around it: comments are kept
declare let ifTrivia: string | null;
/* before */ if (ifTrivia == null) { ifTrivia = 'a'; } // after

// `if` statement with a comment in the test: no fix
if (ifTrivia /* c */ == null) { ifTrivia = 'b'; }

// `if` statement with a comment before the body: no fix
if (ifTrivia == null) /* c */ { ifTrivia = 'c'; }

// `if` statement with a comment in the test, the body, or the assignment: no fix
declare let commentedIf: Date | null;
if /* c */ (!commentedIf) { commentedIf = new Date(); }
if (!commentedIf) /* c */ { commentedIf = new Date(); }
if (!commentedIf) { commentedIf = new Date() /* c */; }
if (!commentedIf) { commentedIf /* c */ = new Date(); }

// Truthiness ternary: comments before the test and before the present branch are kept
declare const truthy: Date | null;
const t14 = /* lead */ truthy ? /* present */ truthy : new Date();

// Truthiness ternary: comments in the dropped parts: no fix
const t15 = truthy /* c */ ? truthy : new Date();
const t16 = truthy ? truthy /* c */ : new Date();

// Truthiness ternary: line breaks around the present branch
export function returned(value: Date | null) {
	return value ?
		value :
		new Date();
}

export function returnedInverted(value: Date | null) {
	return !value ?
		new Date() :
		value;
}

export function returnedWithLineComment(value: Date | null) {
	return value ? // note
		value : new Date();
}

export function thrown(value: Error | null) {
	throw value ?
		value :
		new Error('default');
}

export function formatted(value: Date | null) {
	const result = value
		? value
		: new Date();
	return result;
}
