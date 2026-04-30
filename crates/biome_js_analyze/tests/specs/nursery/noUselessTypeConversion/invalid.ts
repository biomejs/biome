/* should generate diagnostics */
String("asdf");
"asdf".toString();
"" + "asdf";
"asdf" + "";

let str = "asdf";
str += "";

let otherStr = "asdf";
"asdf" + (otherStr += "");

Number(123);
+123;
Boolean(true);
!!true;
BigInt(3n);
~~3n;

function stringIdentity<T extends string>(x: T) {
    return String(x);
}

function numberIdentity<T extends number>(x: T) {
    return Number(x);
}

function booleanIdentity<T extends boolean>(x: T) {
    return Boolean(x);
}

function bigintIdentity<T extends bigint>(x: T) {
    return BigInt(x);
}

String("a" + "b").length;
("a" + "b").toString().length;
2 * +(2 + 2);
2 * Number(2 + 2);
false && !!(false || true);
false && Boolean(false || true);
2n * BigInt(2n + 2n);

~~1;
~~-1;

declare const threeOrFour: 3 | 4;
~~threeOrFour;

declare const threeOrFourBigInt: 3n | 4n;
~~threeOrFourBigInt;

declare const pickedStr: Pick<{ a: string; b: number }, "a">;
String(pickedStr.a);
pickedStr.a.toString();
"" + pickedStr.a;

declare const omittedStr: Omit<{ a: string; b: number }, "b">;
String(omittedStr.a);

declare const pickedNum: Pick<{ n: number; s: string }, "n">;
Number(pickedNum.n);
+pickedNum.n;

declare const pickedBool: Pick<{ flag: boolean; label: string }, "flag">;
Boolean(pickedBool.flag);
!!pickedBool.flag;

function reqStr(x: Required<{s?: string}>) { return String(x.s); }
function reqNum(x: Required<{n?: number}>) { return Number(x.n); }
function reqBig(x: Required<{b?: bigint}>) { return BigInt(x.b); }
function roStr(x: Readonly<{s: string}>) { return String(x.s); }
