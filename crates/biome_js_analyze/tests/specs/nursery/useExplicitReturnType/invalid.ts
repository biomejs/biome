/* should generate diagnostics */

// Function declaration without return type
function test(a: number, b: number) {
  return;
}

// Function declaration without return type and no params
function test2() {
  return;
}

// Function expression without return type
var fn = function () {
  return 1;
};

// Arrow function without return type
var arrowFn = () => 'test';

// Class with missing return types
class Test {
  constructor() {}
  get prop() {
    return 1;
  }
  set prop(value: number) {}
  method() {
    return;
  }
  arrow = () => 'arrow';
  private method2() {
    return;
  }
}

// Untyped arrow (allowTypedFunctionExpressions is true but arrow is untyped)
var arrowFn2 = () => 'test';

// Untyped nested arrow
function fooAny(): any {
  const bar2 = () => () => console.log('aa');
}

// Assignment to any variable
let anyValue: any;
function fooAny2(): any {
  anyValue = () => () => console.log('aa');
}

// Class method with untyped inner arrow
class Foo2 {
  foo(): any {
    const bar2 = () => () => {
      return console.log('foo');
    };
  }
}

// Untyped function expression
var funcExpr = function () {
  return 'test';
};

// Class property with untyped inner arrow
function fooAny3(): any {
  class Foo3 {
    foo = () => () => {
      return console.log('foo');
    };
  }
}

// Higher-order function returning arrow without return type (allowHigherOrderFunctions hardcoded true)
var arrowHof = () => () => {};
var arrowHof2 = () => function () {};
var arrowHof3 = () => {
  return () => {};
};
var arrowHof4 = () => {
  return function () {};
};
function fn1() {
  return () => {};
}
function fn2() {
  return function () {};
}

// Sibling arrow missing type
function fn3() {
  const bar = () => (): number => 1;
  const baz = () => () => 'baz';
  return function (): void {};
}

// Not a higher-order function - returns non-function first
function fn4(arg: boolean) {
  if (arg) return 'string';
  return function (): void {};
}

// Deeply nested higher-order with missing return type at leaf
function FunctionDeclaration() {
  return function FunctionExpression_Within_FunctionDeclaration() {
    return function FunctionExpression_Within_FunctionExpression() {
      return () => {
        // ArrowFunctionExpression_Within_FunctionExpression
        return () =>
          // ArrowFunctionExpression_Within_ArrowFunctionExpression
          () =>
            1; // ArrowFunctionExpression_Within_ArrowFunctionExpression_WithNoBody
      };
    };
  };
}

// Higher-order but inner function doesn't return a function
var hof5 = () => () => {
  return () => {
    return;
  };
};

// as any / as Action are NOT const assertions
const funcAsAny = (value: number) => ({ type: 'X', value }) as any;
const funcAsAction = (value: number) => ({ type: 'X', value }) as Action;

// Class property without type - untyped array
class Bar5 {
  bar = [
    {
      foo: x => x + 1,
    },
  ];
}

// IIFE flagged under default (allowIIFEs: false is default)
const fooIife = (function () {
  return 'foo';
})();
