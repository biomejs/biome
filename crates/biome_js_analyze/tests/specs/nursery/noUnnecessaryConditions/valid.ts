// should not generate diagnostics

// Nullable array - condition is necessary
function head<T>(items: T[] | null) {
  if (items) {  // This check is necessary
    return items[0].toUpperCase();
  }
}

// Union with falsy value - condition is necessary
function foo(arg: 'bar' | 'baz' | null) {
  if (arg) {  // This check is necessary
  }
}

// Optional parameter - optional chaining is necessary
function bar(arg?: string) {
  return arg?.length;  // ?. is necessary
}

// Potentially undefined - optional chaining is necessary
function baz(arg: string | undefined) {
  return arg?.length;  // ?. is necessary
}

// Union type with null/undefined - condition is necessary
interface User {
  name: string;
}
function getUser(): User | null {
  return Math.random() > 0.5 ? { name: "test" } : null;
}
function processUser() {
  const user = getUser();
  return user?.name;  // ?. is necessary
}

// Dynamic conditions are fine
function checkDynamic(condition: boolean, value: string) {
  if (condition && value) {  // This is dynamic, not statically determinable
    return value;
  }
}

// Complex union types
function handleUnion(value: string | number | null) {
  if (value) {  // This check is necessary due to null
    return value.toString();
  }
}

// Boolean variables - condition is necessary
declare const bool1: boolean;
declare const bool2: boolean;

if (bool1) {
  console.log("might run");
}

if (bool1 && bool2) {
  console.log("might run");
}

// Generic type parameters - condition is necessary
function generic<T>(value: T) {
  if (value) {  // T could be anything
    return value;
  }
}

// Any type - condition is necessary
function anyValue(value: any) {
  if (value) {  // any could be falsy
    return value;
  }
}

// Unknown type - condition is necessary
function unknownValue(value: unknown) {
  if (value) {  // unknown could be falsy
    return value;
  }
}

// Union with falsy values - condition is necessary
function unionWithFalsy(value: string | 0 | false) {
  if (value) {  // Could be 0 or false
    return value.toString();
  }
}

// Optional chaining with potentially undefined
interface MaybeUser {
  name?: string;
}
function testOptional(user: MaybeUser) {
  return user.name?.toUpperCase();  // name could be undefined
}

// Optional chaining with nested optional properties
interface NestedOptional {
  inner?: {
    value?: string;
  };
}
function testNested(obj: NestedOptional) {
  return obj.inner?.value?.toUpperCase();  // Both could be undefined
}

// Nullish coalescing with potentially nullish values
function testNullishValid(value: string | null | undefined) {
  return value ?? "default";  // value could be null or undefined
}

// Logical OR with potentially falsy left side
function testOrValid(value: string | 0) {
  return value || "default";  // value could be 0
}

// Comparisons with variables
function testComparisons(a: number, b: number) {
  return a === b;  // Result depends on runtime values
}

// Type predicates
function isString(value: unknown): value is string {
  return typeof value === "string";
}

// Conditional with type narrowing
function testTypeNarrowing(value: string | number) {
  if (typeof value === "string") {  // Necessary for type narrowing
    return value.toUpperCase();
  }
  return value.toString();
}

// Array method conditions
function filterTruthy<T>(items: (T | null | undefined)[]) {
  return items.filter(item => item);  // Necessary to filter out nullish values
}

// Optional chaining with union types containing null/undefined
function testUnionOptional(value: { prop: string } | null) {
  return value?.prop;  // Necessary because value could be null
}

// Loops with dynamic conditions
function testLoop(items: string[]) {
  let i = 0;
  while (i < items.length) {  // Dynamic condition
    console.log(items[i]);
    i++;
  }
}

// Ternary with dynamic condition
function testTernary(condition: boolean) {
  return condition ? "yes" : "no";  // Dynamic condition
}

// Short-circuit evaluation with side effects
function testSideEffects(condition: boolean) {
  return condition && callFunction();  // callFunction might have side effects
}

function callFunction(): boolean {
  console.log("side effect");
  return true;
}

// Truthiness check on union with zero
function testZero(value: number | null) {
  if (value) {  // Necessary - could be null or 0
    return value.toString();
  }
}

// Mixed logical expressions with unknowns
function testMixed(a: boolean, b: boolean) {
  return a || b;  // Both could be false
  return a && b;  // Either could be false
}