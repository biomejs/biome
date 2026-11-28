/* should generate diagnostics */
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

var x: string = "str";

function f(x: number = 1) {}

class X {
	x: number = 1;
}

class X {
	constructor(protected x: number = 1) {}
}
