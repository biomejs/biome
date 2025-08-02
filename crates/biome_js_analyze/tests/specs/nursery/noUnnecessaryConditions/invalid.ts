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