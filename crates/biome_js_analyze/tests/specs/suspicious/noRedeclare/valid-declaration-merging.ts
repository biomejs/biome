// Type and value merging
export type Order = -1 | 0 | 1;
export const Order = {
	LOWER: -1,
	EQUAL: 0,
	UPPER: 1,
} as const;

export type Direction = "Up" | "Down";
export namespace Direction {
	export const Up = "Up";
	export type Up = "Up";
	export const Down = "Down";
	export type Down = "Down";
}

export type Person = {
	readonly name: string;
};
export function Person(name: string): Person {
	return { name };
}

interface Organization {
	readonly name: string;
}
export function Organization(name: string): Organization {
	return { name };
}

// Interface merging
export interface Splitted {
	f(): void;
}
export interface Splitted {
	g(): void;
}

// interface, class, and namespace merging
export interface MoralPerson {
	phantom(): void;
}
export class MoralPerson {
	name: string;
	constructor(name: string) {
		this.name = name;
	}
}
export namespace MoralPerson {
	export function from(name: string) {
		return new MoralPerson(name);
	}
}

// function and namespace merging
export function mod(): void {}
export namespace MoralPerson {
	export function f(): void {}
}

// enum and namespace merging
export enum Orientation {
	North,
	East,
	South,
	West,
}
export namespace Orientation {
	export function f(): void {}
}

// variable and namespace merging
declare namespace bodyParser {
	interface BodyParser {
		/** @deprecated */
		(): void
	}
	interface Options {
		inflate?: boolean | undefined
	}
}
declare const bodyParser: bodyParser.BodyParser

namespace ConcreteNamespaceMergeVar {
    export interface Foo {
        foo: string
    }
}

export const ConcreteNamespaceMergeVar = { foo: 'bar' }
ConcreteNamespaceMergeVar.foo = 'baz'
type Bar = ConcreteNamespaceMergeVar.Foo

// namespace merging
export namespace X {
	export function f(): void {}
}
export namespace X {
	export function g(): void {}
}

// enum merging
export enum Orientation2 {
	North = 0,
	South = 1,
}
export enum Orientation2 {
	East = 2,
	West = 3,
}

// overloads
function f(a: boolean): boolean;
function f(a: number): number;
function f(a: number | boolean): number | boolean {
	return a;
}

export default function(a: boolean): boolean;
export default function(a: number): number;
export default function(a: number | boolean): number | boolean {
	return a;
}

function g(A, { B }) {
	interface A {}
	interface B {}
}

export function h<a>(a: a) {
	return a;
}

// Issue https://github.com/biomejs/biome/issues/2608
type Overloads = {
	(e: "change-kind", { kind }: { kind: "a" }): void;
	(e: "change-kind", { kind, extra }: { kind: "b"; extra: number }): void;
};

// Issue https://github.com/biomejs/biome/issues/2372
type F = number
export default function F() {}