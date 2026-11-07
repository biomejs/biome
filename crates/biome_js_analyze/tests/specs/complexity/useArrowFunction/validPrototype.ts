/* should not generate diagnostics */

// `new` through a type assertion
const asNew = function () {};
new (asNew as any)();

// `.prototype` through `satisfies`
const satisfiesPrototype = function () {};
(satisfiesPrototype satisfies Function).prototype;

// `.prototype` through a non-null assertion
const nonNullPrototype = function () {};
nonNullPrototype!.prototype.x = 0;

// `instanceof` through an angle-bracket assertion
const angleInstanceof = function () {};
const value = {};
value instanceof (<any>angleInstanceof);

// `extends` through a type assertion
const asExtends = function () {};
class Child extends (asExtends as any) {}

// Type assertion around the function expression itself
const assertedFunction = function () {} as any;
assertedFunction.prototype;
