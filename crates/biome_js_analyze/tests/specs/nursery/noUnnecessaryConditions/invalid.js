// should generate diagnostics

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

if (   undefined   ) {
	console.log("never runs");
}


// Unnecessary logical OR with always truthy left side
const result1 = {} || "fallback";  // Left side is always truthy

// Unnecessary logical AND short-circuiting
const result2 = true && "result";  // Left side is always true

// Unnecessary ternary conditions
const result3 = true ? "yes" : "no";  // Condition is always true
const result4 = false ? "yes" : "no";  // Condition is always false

// Unnecessary comparisons with literals
const comp1 = true === true;  // Always true
const comp2 = false === true;  // Always false
const comp3 = 5 > 3;  // Always true
const comp3 = 3 < 5;  // Always true
const comp3 = 1 === 1;  // Always true
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

// Unnecessary nullish coalescing with non-nullish literals
const nullish1 = "hello" ?? "default";  // Left side is never nullish
const nullish2 = 42 ?? 0;  // Left side is never nullish
const nullish3 = {} ?? {};  // Left side is never nullish

// Logical expressions with mixed literals
const mixed1 = true || false;  // Left side makes right irrelevant
const mixed2 = false && true;  // Left side makes right irrelevant
const mixed3 = null ?? "default";  // Left side is always nullish
const mixed4 = undefined ?? "default";  // Left side is always nullish

// Unnecessary optional chaining on literals (if supported)
const optional1 = "hello"?.length;  // String is never nullish
const optional2 = [1, 2, 3]?.[0];  // Array is never nullish

// Function expressions with constant conditions
const fn1 = function() {
  if (true) return "always";
};

const fn2 = () => {
  return false ? "never" : "always";
};

// Nested constant conditions
if (true) {
  if (false) {
    console.log("never runs");
  }
}

// Switch with constant discriminant
switch (true) {
  case true:
    console.log("always matches");
    break;
  case false:
    console.log("never matches");
    break;
}

// Object property access with constants
const obj = {};
if (obj) {  // Object literal is always truthy
  console.log("always runs");
}

// Array access with constants
const arr = [];
if (arr) {  // Array literal is always truthy
  console.log("always runs");
}
