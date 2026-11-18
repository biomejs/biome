/* should not generate diagnostics */
type Rec = Record<string, any>;

interface Empty {}

interface WithProperty {
	bar: string;
}

interface PropertyFirst {
	bar: string;
	[key: string]: any;
}

interface IndexFirst {
	[key: string]: any;
	bar: string;
}

type EmptyLiteral = {};

type LiteralWithProperty = {
	bar: string;
	[key: string]: any;
};

type InGeneric = Generic<{
	[key: string]: any;
	bar: string;
}>;

function withParameter(arg: { [key: string]: any; bar: string }) {}

function withReturnType(): { [key: string]: any; bar: string } {}

class WithIndexSignature {
	[key: string]: any;
}

// Circular references
type CircularUnion = { [key: string]: string | CircularUnion };
type CircularDirect = { [key: string]: CircularDirect };
type CircularOuterUnion = { [key: string]: CircularOuterUnion } | CircularOuterUnion;
type CircularMapped = { [key in string]: CircularMapped };
type CircularParenthesized = { [key: string]: (CircularParenthesized) };

interface CircularInterface {
	[key: string]: CircularInterface;
}

interface CircularGeneric<T> {
	[key: string]: CircularGeneric<T>;
}

interface CircularGenericUnion<T> {
	[key: string]: CircularGenericUnion<T> | string;
}

interface CircularIntersection {
	[s: string]: CircularIntersection & {};
}

interface CircularConditionalCheck<T> {
	[s: string]: CircularConditionalCheck extends T ? string : number;
}

interface CircularConditionalExtends<T> {
	[s: string]: T extends CircularConditionalExtends ? string : number;
}

interface CircularConditionalTrue<T> {
	[s: string]: T extends true ? CircularConditionalTrue : number;
}

interface CircularConditionalFalse<T> {
	[s: string]: T extends true ? string : CircularConditionalFalse;
}

interface CircularIndexedObject {
	[s: string]: CircularIndexedObject[number];
}

interface CircularIndexedIndex {
	[s: string]: {}[CircularIndexedIndex];
}

// Indirect circular references
interface TwoA {
	[key: string]: TwoB;
}

interface TwoB {
	[key: string]: TwoA;
}

interface ThreeA {
	[key: string]: ThreeB;
}

interface ThreeB {
	[key: string]: ThreeC;
}

interface ThreeC {
	[key: string]: Record<string, ThreeA>;
}

type AliasA = {
	[key: string]: AliasB;
};

type AliasB = {
	[key: string]: AliasC;
};

type AliasC = {
	[key: string]: AliasA;
};

interface MixedA {
	[key: string]: MixedB;
}

type MixedB = {
	[key: string]: MixedC;
};

interface MixedC {
	[key: string]: MixedA;
}

// A cycle between other types doesn't stop the search
interface OuterCycle {
	[key: string]: InnerCycleA | OuterCycle;
}

interface InnerCycleA {
	[key: string]: InnerCycleB;
}

interface InnerCycleB {
	[key: string]: InnerCycleA;
}

type ExampleUnion = boolean | number;

type ExampleRoot = ExampleUnion | ExampleObject;

interface ExampleObject {
	[key: string]: ExampleRoot;
}

type CircularMappedGeneric<K extends string = never> = {
	[k in K]: CircularMappedGeneric;
};

type IndirectMapped<K extends string = never> = {
	[k in K]: IndirectMappedAlias;
};

type IndirectMappedAlias = IndirectMapped;

// Mapped types that a `Record` can't express
type KeyInValue = { [key in Foo]: key | number };

function keyInModifiedValue(e: { readonly [key in PropertyKey]-?: key }) {}

function keyofKeys(): {
	[k in keyof ParseResult]: unknown;
} {
	return {};
}

type Renamed = { [key in string as `get${key}`]: number };
