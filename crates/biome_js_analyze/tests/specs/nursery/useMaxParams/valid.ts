/* should not generate diagnostics */
interface MyInterface {
    value: number;
}

function withThisParamValid(this: MyInterface, a: number, b: number, c: number): number {
    return this.value + a + b + c;
}

declare function makeDateValid(m: number, d: number, y: number): Date;

type sumValid = (a: number, b: number, c: number) => number;
