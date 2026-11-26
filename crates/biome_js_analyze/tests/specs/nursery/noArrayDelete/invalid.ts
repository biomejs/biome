// should generate diagnostics
declare const arr: number[];
declare const index: number;

delete arr[0];
delete arr[index];
delete arr[index + 1];
delete arr[arr.length - 1];
delete (arr[0]);
delete (arr)[0];
delete arr?.[0];

declare const genericArr: Array<number>;
delete genericArr[0];

declare const tuple: [number, string];
delete tuple[1];

declare const arrayOrTuple: number[] | [boolean, string];
delete arrayOrTuple[0];

const literalArr = [1, 2, 3];
delete literalArr[1];

declare const obj: { items: number[] };
delete obj.items[0];

declare function getArr(): string[];
delete getArr()[0];

// The fix wraps a comma-separated expression in parentheses.
let a = 0;
let b = 1;
delete arr[a, b];
delete arr[(a, b)];

// Comments before and after the expression are kept.
delete arr[0] /* after */;
delete arr[0]; // trailing
/* leading */ delete arr[0];

// No fix is offered when the expression contains comments.
delete /* before */ arr[0];
delete arr[/* inside */ 0];
delete arr[
	index // last
];
delete arr // array
[0];
delete (/* inside */ arr[0]);

// The returned value is used.
const result = delete arr[0];
