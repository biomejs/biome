/* should generate diagnostics */
!(a === b);
!(a !== b);
!(a == b);
!(a != b);
!(a === b === c);
!(typeof value === "undefined");
!((a === b));
(!(a === b)).toString();
foo(!(a === b));
!(a /* comment */ === b);
!/* comment */(a === b);
!(a === b) /* trailing */;
const isDifferent = !(a === b);
if (!(typeof value === "undefined")) {}
const arrow = () => !(a === b);
x = !(a === b) ? 1 : 2;
x = !(a === b) && c;
x = [!(a === b)];
x = `${!(a === b)}`;
x = !(a?.b === c);
x = !(a?.() == b);
function f1() { return!(a === b); }
function f2() { throw!(a === b); }
switch (foo) { case!(a === b): break; }
async function f3() { return await !(a === b); }
async function f4() { return await!(a === b); }
x = void !(a === b);
x = typeof !(a === b);
x = typeof!(a === b);
x = +!(a === b);
x = a + !(b === c);
x = a === !(b === c);
x = !(a === b) in c;
function* f5() { yield !(a === b); }
function* f6() { yield!(a === b); }
foo(...!(a === b));
!({ value: 1 }.value === 1);
!(function () {} === value);
x = !({ value: 1 }.value === 1);
const read = () => !({ value: 1 }.value === 1);
(!({ value: 1 }.value === 1));
if (x) {} else!(a === b);
