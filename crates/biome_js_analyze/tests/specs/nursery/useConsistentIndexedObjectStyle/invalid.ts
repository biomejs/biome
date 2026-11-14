/* should generate diagnostics */
interface Basic {
	[key: string]: any;
}

interface ReadonlyInterface {
	readonly [key: string]: any;
}

interface GenericInterface<A> {
	[key: string]: A;
}

interface GenericDefault<A = any> {
	[key: string]: A;
}

interface ReadonlyGeneric<A> {
	readonly [key: string]: A;
}

interface TwoGenerics<A, B> {
	[key: A]: B;
}

interface ReadonlyTwoGenerics<A, B> {
	readonly [key: A]: B;
}

interface Extending extends Base {
	[index: number]: unknown;
}

export interface Exported {
	[key: string]: unknown;
}

declare interface Declared {
	[key: string]: unknown;
}

/** Documentation */
interface Documented {
	[key: string]: unknown;
}

type Literal = { [key: string]: any };

type ReadonlyLiteral = { readonly [key: string]: any };

type InGeneric = Generic<{ [key: string]: any }>;

type ReadonlyInGeneric = Generic<{ readonly [key: string]: any }>;

function parameter(arg: { [key: string]: any }) {}

function returnType(): { [key: string]: any } {}

function readonlyParameter(arg: { readonly [key: string]: any }) {}

function readonlyReturnType(): { readonly [key: string]: any } {}

let variable: { [key: string]: number };

// References to other types
type QualifiedValue = { [k: string]: A.QualifiedValue };

type OtherValue = { [key: string]: AnotherFoo };

type NestedLiteral = { [key: string]: { [key: string]: NestedLiteral } };

type UnionWithSelf = { [key: string]: string } | UnionWithSelf;

interface QualifiedInterface {
	[k: string]: A.QualifiedInterface;
}

interface NestedInInterface {
	[k: string]: { [key: string]: NestedInInterface };
}

interface SelfInProperty {
	[key: string]: { foo: SelfInProperty };
}

interface SelfInArray {
	[key: string]: SelfInArray[];
}

interface SelfInFunction {
	[key: string]: () => SelfInFunction;
}

interface SelfInTuple {
	[s: string]: [SelfInTuple];
}

interface ChainA {
	[key: string]: ChainB;
}

interface ChainB {
	[key: string]: ChainC;
}

interface ChainC {
	[key: string]: number;
}

type SelfInPropertyAlias = {
	[key: string]: { foo: SelfInPropertyAlias };
};

type MappedSelfInProperty<K extends string> = {
	[k in K]: { foo: MappedSelfInProperty<K> };
};

// Mapped types
type ReadonlyMapped = { readonly [key in string]: number };

type PlusReadonlyMapped = { +readonly [key in string]: number };

type MinusReadonlyMapped = { -readonly [key in string]: number };

type Mapped = { [key in string]: number };

function optionalMapped(e: { [key in PropertyKey]?: string }) {}

function plusOptionalMapped(e: { [key in PropertyKey]+?: string }) {}

function minusOptionalMapped(e: { [key in PropertyKey]-?: string }) {}

function readonlyRequiredMapped(e: { readonly [key in PropertyKey]-?: string }) {}

type Options = [
	{ [Type in (typeof optionTesters)[number]["option"]]?: boolean } & {
		allow?: TypeOrValueSpecifier[];
	},
];

export type MakeRequired<Base, Key extends keyof Base> = {
	[K in Key]-?: NonNullable<Base[Key]>;
} & Omit<Base, Key>;

function parenthesizedKeyof(): {
	[k in (keyof ParseResult)]: unknown;
} {
	return {};
}

type MappedWithoutValue = {
	[k in string];
};

// Comments that a fix would drop
type CommentInLiteral = { [key: string]: /* preserve me */ number };

interface CommentInInterface {
	// preserve me
	[key: string]: number;
}

type CommentInMapped = {
	// preserve me
	[Key in "a" | "b"]: number;
};

type CommentsInKeys = {
	// 1
	[Key in // 2
		| "a" // 3
		| "b" // 4
		| "c" // 5
		| "d" // 6
	]: string; // 7
	// 8
};

type KeptCommentInValue = { [key: string]: { a: /* kept */ number } };

interface Variance<out T> {
	[key: string]: T;
}
