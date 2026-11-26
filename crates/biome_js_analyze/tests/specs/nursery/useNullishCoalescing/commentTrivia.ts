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
