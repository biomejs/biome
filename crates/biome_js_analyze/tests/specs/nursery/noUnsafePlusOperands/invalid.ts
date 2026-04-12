/* should generate diagnostics */

let arrayAndObject = [] + {};
let arrayAndNumber = [] + 5;
let numberAndArray = 5 + [3];

let bigintAndNumber = 1n + 1;
let numberAndBigint = 1 + 1n;

{
	declare const pair: { first: number; second: string };
	const objectAndString = pair + "!";
}

{
	declare const value: { value: number } & { label: string };
	declare const text: string;
	const intersectionAndString = value + text;
}

{
	interface Value {
		a: 1;
	}

	declare const value: Value;
	declare const text: string;
	const interfaceAndString = value + text;
}

{
	declare const value: unknown;
	declare const text: string;
	const unknownAndString = value + text;
}

{
	declare const value: never;
	declare const text: string;
	const neverAndString = value + text;
}

{
	declare const value: symbol;
	declare const text: string;
	const symbolAndString = value + text;
}

{
	let total = 1n;
	total += 1;
}

{
	let total = 1;
	total += 1n;
}
