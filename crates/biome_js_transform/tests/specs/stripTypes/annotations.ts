function foo<T>(a: T, b?: number, ...rest: string[]): T | undefined {
	let x!: number;
	x = b as number;
	const y = foo<string>;
	return a;
}
const bar = <T,>(a: T): T => a;
function baz(this: Window, a?: boolean) {}
