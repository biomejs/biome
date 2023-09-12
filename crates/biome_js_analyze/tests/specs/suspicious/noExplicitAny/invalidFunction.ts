function generic(): any {}

function generic2(): Array<any> {}

function generic3(): any[] {}

function generic4(param: Array<any>): number {}

function generic5(param: any[]): number {}

function generic6(param: Array<any>): Array<any> {}

function generic7(): Array<Array<any>> {}

function generic8(): Array<any[]> {}

function test<T extends Partial<any>>() {}

function foo(a: number, ...rest: any[]): void {
	return;
}

function foo5(...args: any) {}

function quux5(fn: (...args: any) => void): void {}

function quuz5(): ((...args: any) => void) {}

declare function waldo5(...args: any): void;
