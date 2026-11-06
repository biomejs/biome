/* should generate diagnostics */
declare const value: any;
declare const promisedValue: Promise<any>;

function returnsAssertedAny() {
	return 1 as any;
}

function returnsObjectCreate() {
	return Object.create(null);
}

function returnsJsonParse(text: string) {
	return JSON.parse(text);
}

const arrowWithBlock = () => {
	return 1 as any;
};

const arrowWithExpression = () => Object.create(null);

function returnsAnyArray() {
	return [] as any[];
}

function returnsGenericAnyArray() {
	return [] as Array<any>;
}

function returnsReadonlyAnyArray() {
	return [] as readonly any[];
}

const arrowReturnsAnyArray = () => [] as any[];

const annotatedArrow = (): string[] => [1, 2, 3] as any[];

function returnsAnyUnion() {
	return {} as any | string;
}

function returnsAnyDeclaredPromiseAny(): Promise<any> {
	return {} as any;
}

function returnsAnyDeclaredPromiseObject(): Promise<object> {
	return {} as any;
}

async function asyncReturnsAny() {
	return value;
}

async function asyncReturnsPromiseAny(): Promise<number> {
	return promisedValue;
}

async function asyncReturnsAssertedPromiseAny(arg: number) {
	return arg as Promise<any>;
}

async function asyncReturnsPromiseResolve(): Promise<object> {
	return Promise.resolve<any>({});
}

async function asyncReturnsNestedPromiseAny(): Promise<object> {
	return {} as Promise<Promise<Promise<Promise<any>>>>;
}

async function asyncReturnsPromiseAnyUnion() {
	return {} as Promise<any> | Promise<object>;
}

async function asyncReturnsPromiseOfAnyUnion() {
	return {} as Promise<any | object>;
}

async function asyncReturnsBrandedPromiseAny() {
	return {} as Promise<any> & { __brand: "any" };
}

interface PromiseAlias<T> extends Promise<any> {
	foo: "bar";
}
declare const aliased: PromiseAlias<number>;
async function asyncReturnsExtendedPromiseAny() {
	return aliased;
}

declare function takesNullableCallback(arg: null | (() => any)): void;
takesNullableCallback(() => "foo" as any);

class Container {
	method() {
		return value;
	}

	get getter() {
		return value;
	}
}

const object = {
	method() {
		return value;
	},
};

function outer(): unknown {
	return () => value;
}

function returnsSetOfAny(): Set<string> {
	return new Set<any>();
}

function returnsMapOfAny(): Map<string, string> {
	return new Map<string, any>();
}

function returnsSetOfAnyArray(): Set<string[]> {
	return new Set<any[]>();
}

function returnsNestedSetOfAny(): Set<Set<Set<string>>> {
	return new Set<Set<Set<any>>>();
}

class AnyTypeArgument {
	method(): Set<string> {
		return new Set<any>();
	}
}

type SetFactory = () => Set<string>;
const annotatedVariableArrow: SetFactory = () => new Set<any>();
const annotatedVariableFunction: SetFactory = function test() {
	return new Set<any>();
};

function receiver(arg: SetFactory) {}
receiver(() => new Set<any>());
receiver(function test() {
	return new Set<any>();
});

class Box<T> {
	constructor(public value: T) {}
}
function returnsBoxOfAny(): Box<string> {
	return new Box<any>(1);
}

const untypedObject = { fn: () => value };

const untypedObjectMethod = {
	method() {
		return value;
	},
};

class UntypedProperty {
	callback = () => value;
}

function returnsReadonlyArrayOfAny() {
	return [] as ReadonlyArray<any>;
}

class AnnotatedGetter {
	get getter(): string {
		return value;
	}
}

const annotatedObjectMethod = {
	method(): string {
		return value;
	},
};

(() => value)();
