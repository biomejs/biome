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

// lastIndexOf — presence and absence checks
arr.lastIndexOf(1) !== -1;
arr.lastIndexOf(1) >= 0;
arr.lastIndexOf(1) === -1;
arr.lastIndexOf(1) < 0;
-1 !== arr.lastIndexOf(1);

// some — strict-equality callback
arr.some((item) => item === 1);
arr.some((item) => 1 === item);
arr.some(item => item === 1);
arr.some((item) => { return item === 1; });
arr.some(function (item) { return item === 1; });
