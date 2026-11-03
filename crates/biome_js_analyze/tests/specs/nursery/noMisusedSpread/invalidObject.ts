/* should generate diagnostics */

// Arrays
const fromArrayLiteral = { ...[1, 2, 3] };

const constArray = [1, 2, 3] as const;
const fromConstArray = { ...constArray };

declare const arrayUnion: number[] | string[];
const fromArrayUnion = { ...arrayUnion };

declare function getReadonlyArray(): readonly number[];
const fromReadonlyArray = { ...getReadonlyArray() };

// Iterables
const fromNewSet = { ...new Set([1, 2, 3]) };

declare const setOrObject: Set<number> | { a: number };
const fromSetUnion = { ...setOrObject };

const iterableObject = {
	*[Symbol.iterator]() {
		yield 'test';
	},
};
const fromIterableObject = { ...iterableObject };

declare const iterable: Iterable<string>;
const fromIterable = { ...iterable };

class IterableClass {
	[Symbol.iterator]() {
		return {
			next() {
				return { done: true, value: undefined };
			},
		};
	}
}
const fromIterableClass = { ...new IterableClass() };

// Maps
declare const map: Map<string, number>;
const fromMap = { ...map };
const fromMapWithOther = { other: 1, ...map };

declare const weakMap: WeakMap<{ a: number }, string>;
const fromWeakMap = { ...weakMap };

const fromNewMap = {
	...new Map([
		['test-1', 1],
		['test-2', 2],
	]),
};

// Class instances
const fromDate = { ...new Date() };

declare class HTMLElementLike {}
declare const element: HTMLElementLike;
const fromDeclaredClass = { ...element };

class WithFields {
	a = 1;
	public b = 2;
	private c = 3;
	protected d = 4;
	static e = 5;
}
const fromInstance = { ...new WithFields() };

class Base {
	a = 1;
}
class Derived extends Base {}
const fromDerived = { ...new Derived() };

// Classes
class Declaration {}
const fromClass = { ...Declaration };

const Expression = class {};
const fromClassExpression = { ...Expression };

declare const either: typeof Declaration | typeof Expression;
const fromClassUnion = { ...either };

// Promises
const promise = new Promise(() => {});
const fromPromise = { ...promise };

declare const maybePromise: Promise<number> | { a: number };
const fromPromiseUnion = { ...maybePromise };

function withPromise<P extends Promise<void>>(promise: P) {
	return { ...promise };
}

declare function getPromise(): Promise<number>;
const fromPromiseCall = { ...getPromise() };

// Functions
function declared() {}
const fromFunction = { ...declared };

declare function getFunction(): () => void;
const fromReturnedFunction = { ...getFunction() };

function* generator() {}
const fromGenerator = { ...generator };

interface FunctionWithProps {
	(): string;
	prop: boolean;
}
type FunctionWithoutProps = () => string;
declare const functionUnion: FunctionWithProps | FunctionWithoutProps | object;
const fromFunctionUnion = { ...functionUnion };
