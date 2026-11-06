/* should generate diagnostics */

// Non-null assertions on values that can't be null or undefined.
const literal = 3;
const nonNullLiteral = literal!;

function nonNullParameter(x: number): number {
	return x!;
}

declare const declared: string;
declared!;
(declared)!;
declared! /* trailing */;

declare const nested: { value: string };
nested.value!;

declare const withIndex: { known: string; [key: string]: string };
(withIndex as { known: string }).known!;

function constrained<T extends string>(value: T) {
	return value!;
}

class Example {
	definite!: string;

	method() {
		return this.definite!;
	}
}

let definite!: number;
definite!;

declare let ambient: number;
ambient!;

declare const never: never;
never!;

// Non-null assertions on assignment targets.
let assigned: number | undefined;
assigned! = 1;

declare const target: { value?: number };
target.value! = 1;

// Type assertions that keep the same type.
const angleBracket = <number>(3 + 5);

type Alias = number;
const aliased = (3 + 5) as Alias;

declare function getText(): string;
getText() as string;
<string>getText();
(getText()) as string;
getText() /* comment */ as string;

declare function getUnion(): string | number;
getUnion() as string | number;

declare function getOptional(): string | undefined;
getOptional() as string | undefined;

declare function getBig(): bigint;
getBig() as bigint;

declare function getSymbol(): symbol;
getSymbol() as symbol;

declare function getBoolean(): boolean;
getBoolean() as boolean;

async function awaited(promise: Promise<number>) {
	return (await getNumberAsync()) as number;
}
declare function getNumberAsync(): Promise<number>;

declare const a: number;
declare const b: number;
(a === b) as boolean;
(getText() + getText()) as string;
(getText() as string) as string;

// The fix keeps the parentheses that the operand needs.
const power = getText() + getText() as string < "b";
async function awaitedPower() {
	return await getNumberAsync() as number ** 2;
}

// Chained type assertions that end at the original type.
getText() as unknown as string;
<string>(<unknown>getText());
(getText() as unknown) as string;
(3 + 5) as unknown as number;
const comparison = "a" < <string>(getText() + getText() as unknown);
