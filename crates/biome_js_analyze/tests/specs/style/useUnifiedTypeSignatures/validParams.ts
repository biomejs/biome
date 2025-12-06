/* should not generate diagnostics */

function f(bar: number): string;
function f(barrer: string): string;
function f(whatever: any): any {}

export async function exams(amICooked = 0): Promise<void>;
export async function exams(reason: string): Promise<void>;
export async function exams(foo: any): Promise<any>;

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
