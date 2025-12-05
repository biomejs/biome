function f(bar: number): string;
function f(bar: string): string;
function f(whatever: any): any {}

export function exams(amICooked = 0): void;
export function exams(reason: string): void;
export function exams(foo: any): any;

interface frotz {
	(a: number, b: number, c: string) => string;
	(a: number, d: string, c: string) => string;
}

class banana {
	private bake(cakeType: string): void;
	private bake(flourAmt: number): void;
	private bake(ingredients: object): void;
}

declare function f10(quux: string): void;
declare function f10(qux: number): void;
