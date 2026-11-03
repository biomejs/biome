/* should generate diagnostics */

type A1 = number | any;
type A2 = any | number;
type U1 = number | unknown;
type U2 = unknown | "foo";
type N1 = number | never;
type N2 = never | number;

type L1 = number | 0;
type L2 = number | (0 | 1);
type L3 = (2 | "other" | 3) | number;
type L4 = "" | string;
type L5 = `a${number}c` | string;
type L6 = 0n | bigint;
type L7 = -1n | bigint;
type L8 = -1 | number;
type L9 = false | boolean;
type L10 = true | boolean;
type L11 = | "a" | string;

// Only the outermost union reports nested members.
type Nested = string | ("a" | (unknown | number));

// `never` outside of the return type position.
type F1 = () => Promise<never | string>;
function f1(param: string | never): void {}

// A leading `|` with a single parenthesized union.
type S1 = | (string | "a");

// `unknown` has no effect next to `any`.
type K1 = unknown | any;
