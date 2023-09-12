const x: undefined = undefined; // undefined is an identifier
const x: RegExp = /a/; // RegExp could be redefined

const x: bigint = BigInt(1);
const x: boolean = Boolean(null);
const x: number = Number("1");
const x: RegExp = new RegExp("a");
const x: string = String(1);
const x: symbol = Symbol("a");

let x: string = tag`str`; // could be anything
let x: RegExp = /a/; // RegExp could be redefined

// widen types are allowed in const context
const x: bigint = 1n;
const x: bigint = -1n;
const x: boolean = true;
const x: boolean = false;
const x: number = 1;
const x: number = 1e-5;
const x: string = "str";
const x: string = `str`;
const x: string = `str${f()}`;

class X {
	readonly x: number = 1;
}

class X {
	constructor(readonly x: number = 1) {}
}

// literal types are allowed in non-const contexts
let x: 1n = 1n;
let x: -1n = -1n;
let x: true = true;
let x: false = false;
let x: null = null;
let x: 1 = 1;
let x: -1 = -1;
let x: 1e-5 = 1e-5;
let x: "str" = "str";
let x: "str" = `str`;
let x: "str2" = `str${f()}`;
let x: undefined = void f();
let x: null = null;

function f(x: 1 = 1) {}

class X {
	x: 1 = 1;
}

class X {
	constructor(protected x: 1 = 1) {}
}
