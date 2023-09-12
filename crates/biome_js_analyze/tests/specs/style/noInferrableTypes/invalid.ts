// const contexts
const x/*before*/: /*inside*/ 1 /*after*/ = (1);

const x: 1n = 1n;
const x: -1n = -1n;
const x: false = false;
const x: false = !true;
const x: false = !1;
const x: true = true;
const x: true = !false;
const x: true = !0;
const x: null = null;
const x: 1 = +1;
const x: -1 = -1;
const x: 1e-5 = 1e-5;
const x: RegExp = /a/;
const x: "str" = "str";
const x: "str" = `str`; // constant template string
const x: "str2" = `str${f()}`;
const x: undefined = void f();

class X {
	readonly x: 1 = 1;
}

class X {
	constructor(readonly x: 1 = 1) {}
}

// non-const contexts
let x /*before*/: /*inside*/ number /*after*/ = (1);

let x: bigint = 1n;
let x: bigint = -1n;
let x: boolean = false;
let x: boolean = true;
let x: boolean = !false;
let x: boolean = !true;
let x: number = +1;
let x: number = -1;
let x: number = 1e-5;
let x: RegExp = /a/;
let x: string = "str";
let x: string = `str`;
let x: string = `str${f()}`;
let x: number = +"";
let x: boolean = !"";

function f(x: number = 1) {}

class X {
	x: number = 1;
}

class X {
	constructor(protected x: number = 1) {}
}
