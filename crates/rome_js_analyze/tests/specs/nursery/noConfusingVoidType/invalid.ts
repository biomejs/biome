type PossibleValues = string | number | void;
type MorePossibleValues = string | ((number & any) | (string | void));

function logSomething(thing: void) {}
function printArg<T = void>(arg: T) {}
logAndReturn<void>(undefined);

let voidPromise: Promise<void> = new Promise<void>(() => { });
let voidMap: Map<string, void> = new Map<string, void>();

interface Interface {
	prop: void;
}

class MyClass {
	private readonly propName: void;
}

let foo: void;
let bar = 1 as unknown as void;
let baz = 1 as unknown as void | string;
