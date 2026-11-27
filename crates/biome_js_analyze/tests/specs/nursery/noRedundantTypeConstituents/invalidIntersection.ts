/* should generate diagnostics */

type A1 = number & any;
type A2 = any & "foo";
type N1 = number & never;
type N2 = never & number;
type U1 = number & unknown;
type U2 = unknown & number;

type P1 = number & 0;
type P2 = "" & string;
type P3 = 0n & bigint;
type P4 = -1n & bigint;
type P5 = false & boolean;
type P6 = true & boolean;
type P7 = `a${string}` & string;
type P8 = ("a" | "b") & string;
type P9 = (1 | 2) & ("a" | "b") & string & number;
type P10 = (true | false) & boolean;

// Only the outermost intersection reports nested members.
type Nested = string & ("a" & (unknown & { a: 1 }));

// A leading `&` with a single parenthesized intersection.
type S1 = & (number & 0);

// `any` has no effect next to `never`.
type K1 = any & never;
