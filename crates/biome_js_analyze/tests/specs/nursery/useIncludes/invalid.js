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

// reversed operands
-1 !== arr.indexOf(1);
-1 != arr.indexOf(1);
0 <= arr.indexOf(1);
-1 < arr.indexOf(1);

-1 === arr.indexOf(1);
-1 == arr.indexOf(1);
0 > arr.indexOf(1);

// works with strings too
str.indexOf("world") !== -1;
str.indexOf("world") === -1;

// parenthesized expressions
(arr.indexOf(1)) !== -1;
arr.indexOf(1) !== (-1);
