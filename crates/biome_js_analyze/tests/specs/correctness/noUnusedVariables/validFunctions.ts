/* should not generate diagnostics */

// used function declaration
function a() {}
a();

// we never flag function expressions
(function f() {})

// overloaded functions
function add(a: string, b: string): string;
function add(a: number, b: number): number;
function add(a: any, b: any): any {
	return a + b;
}
add(1, 1);
