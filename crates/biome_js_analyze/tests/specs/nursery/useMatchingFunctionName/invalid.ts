/* should generate diagnostics */
const foo1: () => void = function bar() {};
const foo2 = function bar() {} as () => void;
const foo3 = <() => void>function bar() {};
obj.foo4 = function bar() {} satisfies () => void;
class C1 {
	foo5: () => void = function bar() {};
	@decorator
	foo6 = function bar() {};
}
