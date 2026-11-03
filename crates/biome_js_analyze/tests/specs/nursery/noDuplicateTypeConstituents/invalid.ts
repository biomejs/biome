/* should generate diagnostics */
type I1 = 1 | 1;
type I2 = true & true;
type I3 = null | null;
type I4 = any | any;
type I5 = { a: string | string };
type I6 = { a: string } | { a: string };
type I7 = { a: string; b: number } | { a: string; b: number };
type I8 = Set<string> | Set<string>;
type I9 = string[] | string[];
type I10 = string[][] | string[][];
type I11 = [1, 2, 3] | [1, 2, 3];
type I12 = () => string | string;
type I13 = (arg: string | string) => void;
type I14 = "A" | "A";
type I15 = "A" | 'A';
type I16 = A | B | A;
type I17 = A | B | A | B;
type I18 = (A | B) | (A | B);
type I19 = A | (A | A);
type I20 = (A | B) | A | B | (A | B);
type I21 = (number | string) | number | string;
type I22 = (number & string) & number & string;
type I23 = number & string & (number & string);
type I24 = Record<string, A | A>;
type I25 = (A | B) | ((C | D) & (C | D));
type I26 = A | (A);
type I27 =
	| A
	| B
	| A;
type I28 = Foo<{ a: string }> | Foo<{a:string}>;

// Comments prevent the fix.
type I29 = A | /* comment */ A;

const i30 = (a?: string | undefined) => {};
interface I31 {
	(a?: string | undefined): void;
}
type I32 = new (a?: string | undefined) => void;
function i33(a?: string | undefined) {}
const i34 = function (a?: string | undefined) {};
declare function i35(a?: string | undefined): void;
declare class I36 {
	f(a?: string | undefined): void;
}
interface I37 {
	f(a?: string | undefined): void;
	new (a?: string | undefined): void;
}
type I38 = (a?: string | (undefined | number)) => void;
type I39 = (a?: (undefined | number) | string) => void;
abstract class I40 {
	abstract f(a?: string | undefined): void;
}
function i41(a?: (string | undefined)) {}
function i42(a?: undefined | undefined) {}
