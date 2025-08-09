interface MyInterface {
    value: number;
}

function withThisParam(this: MyInterface, a: number, b: number, c: number, d: number, e: number): number {
    return this.value + a + b + c + d + e;
}

function tooManyParamsWithThis(this: MyInterface, a: number, b: number, c: number, d: number, e: number, f: number): number {
    return this.value + a + b + c + d + e + f;
}

declare function makeDate(m: number, d: number, y: number, h: number, min: number, s: number): Date;

type sum = (a: number, b: number, c: number, d: number, e: number) => number;
