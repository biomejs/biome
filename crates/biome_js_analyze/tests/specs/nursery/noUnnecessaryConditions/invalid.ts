// should generate diagnostics

// Unnecessary conditions on non-nullable arrays
function head<T>(items: T[]) {
  if (items) {  // This check is unnecessary
    return items[0].toUpperCase();
  }
}

// Unnecessary condition on constrained string type
function foo(arg: 'bar' | 'baz') {
  if (arg) {  // This check is unnecessary
  }
}

// Unnecessary optional chaining on non-nullable string
function bar(arg: string) {
  return arg?.length;  // ?. is unnecessary
}

// Unnecessary optional chaining on guaranteed object
interface User {
  name: string;
}
function getUser(): User {
  return { name: "test" };
}
function processUser() {
  const user = getUser();
  return user?.name;  // ?. is unnecessary
}

// Always truthy literal conditions
if (true) {
  console.log("always runs");
}

if (42) {
  console.log("always runs");
}

if ("hello") {
  console.log("always runs");
}

if ({}) {
  console.log("always runs");
}

if ([]) {
  console.log("always runs");
}

// Always falsy literal conditions
if (false) {
  console.log("never runs");
}

if (0) {
  console.log("never runs");
}

if ("") {
  console.log("never runs");
}

if (null) {
  console.log("never runs");
}

if (undefined) {
  console.log("never runs");
}

// Unnecessary nullish coalescing with non-nullable types
function testNullish(str: string) {
  return str ?? "default";  // ?? is unnecessary
}

function testNullish2(num: number) {
  return num ?? 0;  // ?? is unnecessary
}

// Unnecessary logical OR with always truthy left side
function testOr(obj: object) {
  return obj || {};  // || {} is unnecessary
}

// Unnecessary logical AND short-circuiting
function testAnd() {
  return true && "result";  // Left side is always true
}

// Unnecessary ternary conditions
const result1 = true ? "yes" : "no";  // Condition is always true
const result2 = false ? "yes" : "no";  // Condition is always false

// Unnecessary comparisons with literals
const comp1 = true === true;  // Always true
const comp2 = false === true;  // Always false
const comp3 = 5 > 3;  // Always true
const comp4 = "a" === "b";  // Always false

// While loops with constant conditions
while (true) {
  console.log("infinite loop");
  break;
}

while (false) {
  console.log("never runs");
}

// Do-while with constant conditions
do {
  console.log("runs once");
} while (false);

// For loops with constant conditions
for (; true;) {
  break;
}

for (; false;) {
  console.log("never runs");
}

// Unnecessary optional chaining on arrays
function testArray(arr: string[]) {
  return arr?.[0];  // ?. is unnecessary
}

// Unnecessary optional chaining on function calls
function testCall(fn: () => string) {
  return fn?.();  // ?. is unnecessary
}

// Logical expressions with mixed literals
const mixed1 = true || false;  // Left side makes right irrelevant
const mixed2 = false && true;  // Left side makes right irrelevant
const mixed3 = null ?? "default";  // Left side is always nullish

// Always truthy Pick/Omit object
declare const pickedObj: Pick<{a: string}, "a">;
if (pickedObj) console.log(pickedObj.a);

declare const omitObj: Omit<{a: string, b: number}, "b">;
if (omitObj) console.log();

// Always truthy Readonly/Partial object
declare const ro: Readonly<{a: string}>;
if (ro) console.log();

declare const partialObj2: Partial<{a: string}>;
if (partialObj2) console.log();

// Falsy detection via logical-not on always-truthy operand
const arrNotFalsy = [];
if (!arrNotFalsy) {  // always false, should be flagged
  console.log("never runs");
}

const objNotFalsy = {};
if (!objNotFalsy) {  // always false, should be flagged
  console.log("never runs");
}

// Truthy detection via double-not or not-falsy literal
if (!false) {  // always true, should be flagged
  console.log("always runs");
}

if (!0) {  // always true, should be flagged
  console.log("always runs");
}

// Type-aware logical OR on member access
interface Config {
  items: string[];
  name: string;
  getCount(): number;
}

function unnecessaryOrOnMember(cfg: Config) {
  return cfg.items || [];  // cfg.items is string[], always truthy — unnecessary
}

// if-condition on always-truthy member (array is always truthy)
function unnecessaryIfOnMember(cfg: Config) {
  if (cfg.items) {  // cfg.items is string[], always truthy — flag
    return cfg.items[0];
  }
}

// x: string compared with null — always false
function cmpNullWithString(x: string) {
  return x === null;  // always false
}

// x: string compared with undefined — always false
function cmpUndefinedWithString(x: string) {
  return x !== undefined;  // always true
}

// Switch on literal union with a case outside the union — unreachable
function switchOnUnionWithUnreachable(value: 'a' | 'b' | 'c') {
  switch (value) {
    case 'a': return 1;
    case 'd': return 2;  // unreachable — 'd' is not in the union
  }
}

// Switch on literal true with case false — unreachable
function switchOnTrue() {
  switch (true) {
    case false: return 1;  // unreachable
    case true: return 2;
  }
}

// Switch on literal number with impossible case — unreachable
function switchOnSpecificNumber(value: 1 | 2 | 3) {
  switch (value) {
    case 4: return 4;  // unreachable
  }
}

// Unreachable `case` with non-decimal numeric literals (hex, binary, octal, separators).
function switchOnNonDecimalLiterals(value: 1 | 2 | 3) {
  switch (value) {
    case 0xFF: return "hex";      // 255 — unreachable
    case 0b1010: return "bin";    // 10 — unreachable
    case 0o17: return "oct";      // 15 — unreachable
    case 1_000: return "sep";     // 1000 — unreachable
  }
}

// Double-not on always-truthy types — still flagged (recursion inverts twice)
function doubleNotOnArray() {
  const xs: number[] = [];
  if (!!xs) {  // always true, should be flagged
    return xs;
  }
}

// && with always-truthy member access — flagged
interface AlwaysItems {
  items: string[];
}
function andOnAlwaysTruthy(cfg: AlwaysItems) {
  return cfg.items && cfg.items[0];  // cfg.items is string[] always truthy, && is redundant
}

// Ternary with always-truthy member condition — flagged
function ternaryOnAlwaysTruthy(cfg: AlwaysItems) {
  return cfg.items ? "yes" : "no";  // always "yes"
}
