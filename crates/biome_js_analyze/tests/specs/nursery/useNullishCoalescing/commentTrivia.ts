// Test cases for comment trivia preservation around ||
// Using types that are safe for replacement (only truthy or nullish)

declare const a: object | null;
declare const b: object | undefined;

// Comments should be preserved when replacing || with ??
const x = a /* inline comment */ || {};
const y = a || /* inline comment */ {};
const z = a /* before */ || /* after */ {};

// Leading and trailing comments on the operator
const w = b
  // line comment before
  || {};

const v = b ||
  // line comment after
  {};
