/* should not generate diagnostics */

// Function with return type
function test(): void {
  return;
}

// Function expression with return type
var fn = function (): number {
  return 1;
};

// Arrow function with return type
var arrowFn = (): string => 'test';

// Class with typed methods
class Test {
  constructor() {}
  get prop(): number {
    return 1;
  }
  set prop(value: number) {}
  method(): void {
    return;
  }
  arrow = (): string => 'arrow';
}

// Setter without return type is ok
const myObj = {
  set myProp(val) {
    this.myProp = val;
  },
};

// allowTypedFunctionExpressions (hardcoded true) - typed variable declarator
var arrowFn2: Foo = () => 'test';
var funcExpr: Foo = function () {
  return 'test';
};

// Type assertions
const x1 = (() => {}) as Foo;
const x2 = <Foo>(() => {});
const x3 = {
  foo: () => {},
} as Foo;
const x4 = <Foo>{
  foo: () => {},
};
const x5: Foo = {
  foo: () => {},
};

// Nested typed object expressions (https://github.com/typescript-eslint/typescript-eslint/issues/2864)
const x6 = {
  foo: { bar: () => {} },
} as Foo;
const x7 = <Foo>{
  foo: { bar: () => {} },
};
const x8: Foo = {
  foo: { bar: () => {} },
};

// Class property with type annotation (https://github.com/typescript-eslint/typescript-eslint/issues/484)
type MethodType = () => void;
class App {
  private method: MethodType = () => {};
}

// allowHigherOrderFunctions (hardcoded true) - arrow returning typed arrow
var hof1 = () => (): void => {};
var hof2 = () => function (): void {};

// Arrow with block returning typed arrow/function
var hof3 = () => {
  return (): void => {};
};
var hof4 = () => {
  return function (): void {};
};

// Arrow with block and extra statements, returning typed function
var hof5 = () => {
  const foo = 'foo';
  return function (): string {
    return foo;
  };
};

// Function declaration returning typed arrow/function (single statement)
function fn1() {
  return (): void => {};
}
function fn2() {
  return function (): void {};
}

// Function with nested HOF
function fn3() {
  const bar = () => (): number => 1;
  return function (): void {};
}

// Function with conditional returns
function fn4(arg: boolean) {
  if (arg) {
    return () => (): number => 1;
  } else {
    return function (): string {
      return 'foo';
    };
  }

  return function (): void {};
}

// Deeply nested higher-order
function FunctionDeclaration() {
  return function FunctionExpression_Within_FunctionDeclaration() {
    return function FunctionExpression_Within_FunctionExpression() {
      return () => {
        // ArrowFunctionExpression_Within_FunctionExpression
        return () =>
          // ArrowFunctionExpression_Within_ArrowFunctionExpression
          (): number =>
            1; // ArrowFunctionExpression_Within_ArrowFunctionExpression_WithNoBody
      };
    };
  };
}

// Triple nested arrow HOF
var hof6 = () => () => {
  return (): void => {
    return;
  };
};

// Typed call expressions (https://github.com/typescript-eslint/typescript-eslint/issues/679)
declare function foo(arg: () => void): void;
foo(() => 1);
foo(() => {});
foo(() => null);
foo(() => true);
foo(() => '');

// Optional chaining with callbacks
declare function bar(arg: () => void): void;
bar?.(() => 1);
bar?.bar2(() => {});
bar?.bar2?.(() => null);
bar.bar2?.(() => true);
bar?.(() => '');

// Class method callback
class Accumulator {
  private count: number = 0;
  public accumulate(fn: () => number): void {
    this.count += fn();
  }
}
new Accumulator().accumulate(() => 1);

// Object method callbacks with typed argument
declare function fooObj(arg: { meth: () => number }): void;
fooObj({
  meth() {
    return 1;
  },
});
fooObj({
  meth: function () {
    return 1;
  },
});
fooObj({
  meth: () => {
    return 1;
  },
});

// Const assertion in arrow functions (allowDirectConstAssertionInArrowFunctions hardcoded true)
const func1 = (value: number) => ({ type: 'X', value }) as const;
const func2 = (value: number) => ({ type: 'X', value }) as const;
const func3 = (value: number) => x as const;
const func4 = (value: number) => x as const;

// Const assertion with satisfies
interface R {
  type: string;
  value: number;
}
const func5 = (value: number) => ({ type: 'X', value }) as const satisfies R;
const func6 = (value: number) =>
  ({ type: 'X', value }) as const satisfies R satisfies R;
const func7 = (value: number) =>
  ({ type: 'X', value }) as const satisfies R satisfies R satisfies R;

// New expression with callbacks
new Promise(resolve => {});
new Foo(1, () => {});

// Higher-order typed function expressions
type HigherOrderType = () => (arg1: string) => (arg2: number) => string;
const xHof: HigherOrderType = () => arg1 => arg2 => 'foo';

// Interface HOF
interface FooInterface {
  foo: string;
  arrowFn: () => string;
}
function fooHof(): FooInterface {
  return {
    foo: 'foo',
    arrowFn: () => 'test',
  };
}

// Generic HOF
type FooType = (arg1: string) => string;
type BarType<T> = (arg2: string) => T;
const xGeneric: BarType<FooType> = arg1 => arg2 => arg1 + arg2;

// Class property with type annotation (no options)
class Bar3 {
  bar: Foo = {
    foo: x => x + 1,
  };
}

// Class property with typed array
class Bar4 {
  bar: Foo[] = [
    {
      foo: x => x + 1,
    },
  ];
}

// Default parameter with type annotation
type CallBack = () => void;
function f1(gotcha: CallBack = () => {}): void {}
const f2 = (gotcha: CallBack = () => {}): void => {};

// Default parameter with object type annotation
type ObjectWithCallback = { callback: () => void };
const f3 = (gotcha: ObjectWithCallback = { callback: () => {} }): void => {};
