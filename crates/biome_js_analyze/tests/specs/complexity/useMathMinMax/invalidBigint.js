/* should generate diagnostics */
// number versions of bigint expressions
foo < -1 ? -1 : foo;
foo > 2 ** 62 - 1 ? 2 ** 62 - 1 : foo;

// methods that aren't on BigInt
foo > Other.asIntN(64, bar) ? Other.asIntN(64, bar) : foo;
foo > BigInt.other(64, bar) ? BigInt.other(64, bar) : foo;

// expressions whose possible values are all numbers
foo > (bar ?? 1) ? (bar ?? 1) : foo;
foo > (bar ? 1 : 2) ? (bar ? 1 : 2) : foo;

// only `const` variables are followed inside an expression
let notConst = 1n;
foo < -notConst ? -notConst : foo;

// only one level of variables is followed
const firstLevel = 1n;
const secondLevel = firstLevel;
foo < -secondLevel ? -secondLevel : foo;

// call arguments and function bodies are not searched
foo > Number(1n) ? Number(1n) : foo;
foo > (() => 1n) ? (() => 1n) : foo;

// declarations that refer to each other
const cycleA = cycleB;
const cycleB = cycleA;
foo > cycleA ? cycleA : foo;
