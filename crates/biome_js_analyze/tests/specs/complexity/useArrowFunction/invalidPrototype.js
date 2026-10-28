// Using properties other than `prototype` is fine
const otherProperty = function () {};
console.log(otherProperty.name, otherProperty.length);

// `prototype` of another object
const otherObject = function () {};
console.log(otherObject().prototype);

const asMember = function () {};
console.log(Foo[asMember].prototype);

// The function is only passed somewhere else or called
const passed = function () {};
register(passed);
passed.call(null);

// `f` is on the left side of `instanceof`
const instanceofLeft = function () {};
console.log(instanceofLeft instanceof Function);

// `f` is passed to a constructor, not constructed
const constructorArgument = function () {};
new Foo(constructorArgument);

// Async functions don't have a `prototype`, so the fix doesn't change it
const asyncFunction = async function () {};
console.log(asyncFunction.prototype);

// Object properties are not tracked
const object = { method: function () {} };

// A different binding with the same name
const shadowed = function () {};
function scope() {
	const shadowed = {};
	return shadowed.prototype;
}

// The condition of a conditional expression isn't its value
const conditionTest = function () {} ? a : b;
conditionTest.prototype;

// A conditional initializer whose binding doesn't need a `prototype`
const conditionalName = cond ? function () {} : fallback;
conditionalName.name;
