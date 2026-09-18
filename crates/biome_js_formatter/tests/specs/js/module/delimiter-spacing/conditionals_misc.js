// =====================
// Await expressions with grouping parentheses
// =====================

// Await with parentheses
async function foo1() {
	return await (a || b);
}

// Await in expression (formatter adds parens)
async function foo2() {
	const a = await b + await c;
}

// =====================
// Extends clause with parentheses
// =====================

// Extends with call expression
class A extends mixin(Base) {}

// Extends with grouping expression
class B extends (a ? C : D) {}

// =====================
// Require expressions
// =====================

// Basic require
const foo3 = require("module");

// Require with path
const foo4 = require("./path/to/module");

// Require with variable
const foo5 = require(a);

// Require in destructuring
const { a, b } = require("module");

// Require in array destructuring
const [c] = require("module");

// =====================
// Function declarations
// =====================

// Function declaration with parameters
function foo6(a, b, c) {}

// Function declaration with object destructuring
function foo7({a, b}) {}

// Function declaration with array destructuring
function foo8([a, b]) {}

// Async function declaration
async function foo9(a) {}

// Generator function declaration
function* foo10(a) {}

// Async generator function declaration
async function* foo11(a) {}
