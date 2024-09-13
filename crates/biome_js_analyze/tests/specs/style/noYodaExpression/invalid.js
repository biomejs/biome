if ("red" == value) {}
if (true === value) {}
if (5 != value) {}
if (5n != value) {}
if (null !== value) {}
if ("red" <= value) {}
if (`red` <= value) {}
if (`red` <= `${foo}`) {}
if (`red` <= `${"red"}`) {}
if (true >= value) {}
var foo = (5 < value) ? true : false
function foo() { return (null > value); }
if (-1 < str.indexOf(substr)) {}
if ( /* a */ 0 /* b */ < /* c */ foo /* d */ ) {}
if (((((((5)))))) === ((((((((((foo))))))))))) {}
while (0 === (a));
while (0 === (a = b));
async function foo() { return 1 < await bar() }
function *foo() { yield(1) < a }
function *foo() { yield((1)) < a }
function *foo() { yield 1 < a }
function *foo() { yield/**/1 < a }
function *foo() { yield(1) < ++a }
function *foo() { yield(1) < (a) }
x=1 < a
0 < f()in obj
1 > x++instanceof foo
false <= ((x))in foo
false <= ((x)) in foo
1 > x===foo
1 > x
