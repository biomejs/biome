/* should not generate diagnostics */

// Array and argument spreads
const fromArray = [...[1, 2, 3]];
const fromConstArray = [...([1, 2, 3] as const)];

declare const anyData: any;
const fromAny = [...anyData];

declare const unknownData: unknown;
const fromUnknown = [...unknownData];

const numbers = [1, 2, 3];
const copy = [...numbers];

declare function getTuple(): readonly number[];
const fromTuple = [...getTuple()];

const iterator = {
	*[Symbol.iterator]() {
		yield 1;
		yield 2;
		yield 3;
	},
};
const fromIterator = [...iterator];

declare const iterableOrArray: Iterable<number> | number[];
const fromIterableUnion = [...iterableOrArray];

declare const iterableAndArray: Iterable<number> & number[];
const fromIterableIntersection = [...iterableAndArray];

declare function getIterable(): Iterable<number>;
const fromIterableCall = [...getIterable()];

declare const bytes: Uint8Array;
const fromBytes = [...bytes];

declare function takeNumbers(...values: number[]): void;
takeNumbers(...numbers);

// Object spreads
const fromObjectLiteral = { ...{ a: 1, b: 2 } };
const fromConstObject = { ...({ a: 1, b: 2 } as const) };

declare const anyObject: any;
const fromAnyObject = { ...anyObject };

declare const objectOrAny: { a: number; b: number } | any;
const fromObjectOrAny = { ...objectOrAny };

declare const objectAndAny: { a: number; b: number } & any;
const fromObjectAndAny = { ...objectAndAny };

const plain = { a: 1, b: 2 };
const fromPlain = { ...plain };

declare function getObject(): { a: number; b: number };
const fromObjectCall = { ...getObject() };

interface Empty {}
declare const empty: Empty;
const fromInterface = { ...empty };

const fromString = { ...'test' };

declare const promiseLike: PromiseLike<number>;
const fromPromiseLike = { ...promiseLike };

async function awaited() {
	const promise = Promise.resolve({ a: 1 });
	return { ...(await promise) };
}

declare const functionWithProps: {
	(): string;
	prop: boolean;
};
const fromFunctionWithProps = { ...functionWithProps };
