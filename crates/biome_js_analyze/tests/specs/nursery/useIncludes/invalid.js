const arr = [1, 2, 3];
const str = "hello world";

// presence checks — all should be flagged
arr.indexOf(1) !== -1;
arr.indexOf(1) != -1;
arr.indexOf(1) >= 0;
arr.indexOf(1) > -1;

// absence checks — all should be flagged
arr.indexOf(1) === -1;
arr.indexOf(1) == -1;
arr.indexOf(1) < 0;
arr.indexOf(1) <= -1;

// reversed operands
-1 !== arr.indexOf(1);
-1 != arr.indexOf(1);
0 <= arr.indexOf(1);
-1 < arr.indexOf(1);

-1 === arr.indexOf(1);
-1 == arr.indexOf(1);
0 > arr.indexOf(1);
-1 >= arr.indexOf(1);

// works with strings too
str.indexOf("world") !== -1;
str.indexOf("world") === -1;

// parenthesized expressions
(arr.indexOf(1)) !== -1;
arr.indexOf(1) !== (-1);

// lastIndexOf — same semantics as indexOf for presence checks
arr.lastIndexOf(1) !== -1;
arr.lastIndexOf(1) != -1;
arr.lastIndexOf(1) >= 0;
arr.lastIndexOf(1) > -1;

arr.lastIndexOf(1) === -1;
arr.lastIndexOf(1) == -1;
arr.lastIndexOf(1) < 0;
arr.lastIndexOf(1) <= -1;

// reversed operands with lastIndexOf
-1 !== arr.lastIndexOf(1);
-1 === arr.lastIndexOf(1);

// lastIndexOf with strings
str.lastIndexOf("world") !== -1;
str.lastIndexOf("world") === -1;
