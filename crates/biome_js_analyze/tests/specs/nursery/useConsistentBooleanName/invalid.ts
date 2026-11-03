/* should generate diagnostics */

// Boolean type annotations without a prefix
const annotated: boolean = getValue();
declare const declared: boolean;
const literal: true | false = true;
const nullable: boolean | undefined = undefined;
const asserted = value as boolean;
const satisfied = value satisfies boolean;
const angle = <boolean>value;
function download(showProgress: boolean) {}
function optional(completed?: boolean) {}
class Account {
	constructor(private active: boolean) {}
}
function annotatedReturn(): boolean {
	return getValue();
}
async function promiseReturn(): Promise<boolean> {
	return getValue();
}
function predicate(value: unknown): value is string {
	return typeof value === "string";
}
const callback: () => boolean = getCallback();
const nullableCallback: (() => boolean) | undefined = getCallback();
declare function useMyFlag(): boolean;
declare function ambient(): boolean;

// Non-boolean type annotations with a prefix
const isText: string = getValue();
const isPromise: Promise<boolean> = getValue();
function hasTitle(): string | false {
	return false;
}
async function isReady(): Promise<string> {
	return "yes";
}
function isAsserted(value: unknown): asserts value {}
declare function useIsCool(): number;
function isVoid(): void {}
function setCount(isCount: number) {}

// Overloads are reported once, on the implementation
function overloaded(value: string): boolean;
function overloaded(value: number): boolean;
function overloaded(value: unknown): boolean {
	return true;
}

// Default export overloads are reported once, on the implementation
export default function defaultOverloaded(value: string): boolean;
export default function defaultOverloaded(value: unknown): boolean {
	return true;
}
