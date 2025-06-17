type Day =
	| 'Monday'
	| 'Tuesday'
	| 'Wednesday'
	| 'Thursday'
	| 'Friday'
	| 'Saturday'
	| 'Sunday';

const day: Day = 'Monday';
let result = 0;

switch (day) {
	case 'Monday': {
		result = 1;
		break;
	}
}

switch (day) {
}

type A = 'a';
type B = 'b';
type C = 'c';
type Union = A | B | C;

function test(value: Union): number {
	switch (value) {
		case 'a':
			return 1;
	}
}

const A = 'a';
const B = 1;
const C = true;

type Union2 = typeof A | typeof B | typeof C;

function test2(value: Union2): number {
	switch (value) {
		case 'a':
			return 1;
	}
}

type DiscriminatedUnion = { type: 'A'; a: 1 } | { type: 'B'; b: 2 };

function test3(value: DiscriminatedUnion): number {
	switch (value.type) {
		case 'A':
			return 1;
	}
}

declare const value: 'literal';
switch (value) {
}

declare const value2: 'literal' & { _brand: true };
switch (value2) {
}

declare const value3: ('literal' & { _brand: true }) | 1;
switch (value3) {
	case 'literal':
		break;
}

declare const value4: '1' | '2' | number;
switch (value4) {
	case '1':
		break;
}

declare const value5: (string & { foo: 'bar' }) | '1' | 1 | null | undefined;
switch (value5) {
}
