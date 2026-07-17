// =====================
// Parameters (hug layout)
// =====================

// Object binding parameter - single property
function foo1({a}) {}

// Object binding parameter - multiple properties
function foo2({a, b, c}) {}

// Object with default value
function foo3({a = 1}) {}

// Object with rest element
function foo4({a, ...b}) {}

// Object with nested pattern
function foo5({a: {b}}) {}

// Object with aliased property
function foo6({a: b}) {}

// Array binding parameter - single element
function foo7([a]) {}

// Array binding parameter - multiple elements
function foo8([a, b, c]) {}

// Array with default value
function foo9([a = 1]) {}

// Array with rest element
function foo10([a, ...b]) {}

// Array with nested pattern
function foo11([[a]]) {}

// Array with holes
function foo12([, a]) {}

// Mixed: object with nested array
function foo13({a, b: [c]}) {}

// Mixed: array with nested object
function foo14([{a}]) {}

// Empty object pattern
function foo15({}) {}

// Empty array pattern
function foo16([]) {}

// Arrow function with object
const fn1 = ({a}) => {};

// Arrow function with array
const fn2 = ([a]) => {};

// Arrow function with multiple props
const fn3 = ({a, b}) => {};

// Object method with object param
const obj = {
	foo({a}) {}
};

// Class method with object param
class A {
	foo({a}) {}
}

// Constructor with object param
class B {
	constructor({a}) {}
}

// Static method with object param
class C {
	static foo({a}) {}
}

// Async function with object param
async function foo17({a}) {}

// Generator function with object param
function* foo18({a}) {}

// Async arrow with object param
const fn4 = async ({a}) => {};

// With comments inside
function foo19({/* comment */ a}) {}

// With comment at end
function foo20({a /* comment */}) {}

// =====================
// Parameters (no hug)
// =====================

// Multiple parameters
function foo21(a, b) {}

// Multiple params with first being object
function foo22({a}, b) {}

// Multiple params with second being object
function foo23(a, {b}) {}

// Simple identifier parameter
function foo24(a) {}

// Rest parameter only
function foo25(...a) {}

// =====================
// Catch declarations
// =====================

// Basic catch with identifier
try {} catch (e) {}

// Catch with object destructuring
try {} catch ({a}) {}

// Catch with object destructuring multiple properties
try {} catch ({a, b}) {}

// Catch with object destructuring and default
try {} catch ({a = "error"}) {}

// Catch with object destructuring and rest
try {} catch ({a, ...b}) {}

// Catch with array destructuring
try {} catch ([a]) {}

// Catch with array destructuring multiple elements
try {} catch ([a, b]) {}

// Catch with array destructuring and rest
try {} catch ([a, ...b]) {}

// Catch with nested destructuring
try {} catch ({a: {b}}) {}

// Catch with mixed destructuring
try {} catch ({a: [b]}) {}

// =====================
// Boundary tests
// =====================

// Boundary test (fits): 19 properties a-s → 80 chars with delimiter spacing (fits on line)
function foo26({a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s}) {}

// Boundary test (breaks): 20 properties a-t → 83 chars with delimiter spacing (breaks)
function foo27({a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t}) {}
