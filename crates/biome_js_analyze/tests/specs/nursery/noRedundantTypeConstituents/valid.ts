/* should not generate diagnostics */

type T1 = any;
type T2 = never;
type T3 = unknown;
type T4 = 1 | 2;
type T5 = "a" | "b";
type T6 = 1n | 2n;
type T7 = false | true;
type T8 = 1 | false;
type T9 = string | null;
type T10 = number | undefined;
type T11 = bigint | null;
type T12 = string & null;
type T13 = `${string}` & null;
type T14 = string & { readonly brand: unique symbol };
type T15 = "a" | "b" | (string & {});
type T16 = ("a" | 1) & string;
type T17 = string | number;
type T18 = `prefix-${string}` | number;
type T19 = | "a";
type T20 = & "a";
type T21 = | any;

// `never` is allowed in return types.
type F1 = () => never | string;
type F2 = () => (string | never);
type F3 = new () => string | never;
type F4 = { (): string | never };
type F5 = { new (): string | never };
type F6 = { method(): string | never };
type F7 = { get value(): string | never };
declare function f1(): never | "foo";
function f2(): string | never {
	return "";
}
const f3 = (): string | never => "";
class C {
	method(): string | never {
		return "";
	}
	get value(): string | never {
		return "";
	}
}
const o = {
	get value(): string | never {
		return "";
	},
};
abstract class D {
	abstract get value(): string | never;
}
