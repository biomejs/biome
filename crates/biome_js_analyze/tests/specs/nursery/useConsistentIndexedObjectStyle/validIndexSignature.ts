/* should not generate diagnostics */
type Unrelated = Misc<string, unknown>;

type NoArguments = Record;

type OneArgument = Record<string>;

type ThreeArguments = Record<string, number, unknown>;

type Literal = { [key: string]: any };

type InGeneric = Generic<{ [key: string]: any }>;

function parameter(arg: { [key: string]: any }) {}

function returnType(): { [key: string]: any } {}

type Qualified = A.B;

type QualifiedRecord = A.Record<string, any>;

interface Interface {
	[key: string]: any;
}

type Mapped = { [key in string]: unknown };

function shadowed() {
	type Record<K, V> = Map<K, V>;
	type Local = Record<string, any>;
}
