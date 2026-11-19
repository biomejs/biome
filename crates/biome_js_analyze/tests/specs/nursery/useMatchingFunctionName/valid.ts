/* should not generate diagnostics */
const foo1: () => void = function foo1() {};
const foo2 = function foo2() {} as () => void;
class C1 {
	foo3: () => void = function foo3() {};
	private foo4 = function foo4() {};
	declare foo5: () => void;
}
