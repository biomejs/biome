/* should generate diagnostics */
const literal = [...'test'];

function withText<Text extends string>(text: Text) {
	return [...text];
}

declare const stringOrArray: string | number[];
const fromUnion = [...stringOrArray];

declare const branded: string & { __brand: 'test' };
const fromBranded = [...branded];

declare function getString(): string;
const fromCall = [...getString()];

declare function textIdentity(...args: string[]): void;
declare const text: string;
textIdentity(...text);

declare function getGenericString<T extends string>(): T;
const fromGenericCall = [...getGenericString()];
