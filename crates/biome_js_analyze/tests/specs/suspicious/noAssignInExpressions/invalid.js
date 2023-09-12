{
	let a;
	(a += 1) + 2;
}

{
	let a, b;
	a = (b /*before*/ = /*after*/ 1) + 1;
}

{
	let a, b;
	a = ((b = 1), a);
}

{
	let a, b;
	a = (b = 1, b = 2);
}

{
	let a, b;
	a = (class {}, b = 2, function() {});
}

{
	let a;
	const b = (a = 0) ? 1 : 0;
}

{
	let a, b;
	const c = a && (b = 0) ? 1 : 0;
}

function f(a) {
	return (a = 5 + 1);
}

if (a = 0) {
}

if (a || (a = b)) {
}

if (a += b) {
}

while ((a = 0)) {}

while (a *= b) {}

do {} while ((a = a + 1));

do {} while (((a -= 1), a));

do {} while (((a = a + 1), a));

do {} while (a || (a = b));

for (let a = 5; (a = 0); i--) {}

for (let x = 0; (x += 1); ) {}

for (let l; typeof l === "undefined" ? (l = 0) : l; i++) {}

for (; (a = y); ) {}

for (let a = (b = 1); a < 5; ) {}

switch (foo) {
	case (a = b):
		bar();
}

switch (foo) {
	case baz + (a = b):
		bar();
}

((3496.29/*1*/)/*2*/.bkufyydt/*3*/ = /*4*/2e308/*5*/)/*6*/ ? foo : bar;

res.onAborted(() => /*0*/(/*1*/(/*2*/a/*3*/./*4*/b/*5*/)/*6*/ /*7*/ = /*8*/ /*9*/true/*10*/));


(/*1*/[/*2*/a/*3*/,/*4*/b/*5*/, /*6*/c/*7*/]/*8*//*9*/=/*10*/ 2e308) ? foo : bar;
(/*1*/{/*2*/a/*3*/,/*4*/b/*5*/, /*6*/c/*7*/}/*8*//*9*/=/*10*/ 2e308) ? foo : bar;
