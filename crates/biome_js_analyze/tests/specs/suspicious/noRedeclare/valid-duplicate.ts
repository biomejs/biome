const obj = {
	a: 1,
	a: 2,
};

function f(x: number, x: number): void {}

class A {
	g(x: number): number;
	g(x: string): string;
	g(x: number | string): number | string {
		return x;
	}

	f(): void {}
	f(): void {}
}

let a: { [key: string]: string };
let b: { [key: string]: string };
