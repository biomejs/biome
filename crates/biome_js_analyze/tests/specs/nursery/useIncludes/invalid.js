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

// lastIndexOf presence/absence checks
arr.lastIndexOf(1) !== -1;
arr.lastIndexOf(1) >= 0;
arr.lastIndexOf(1) === -1;
-1 !== arr.lastIndexOf(1);
str.lastIndexOf("world") !== -1;

// some() with a strict-equality callback
arr.some(x => x === 1);
arr.some(x => 1 === x);
arr.some((x) => x === 1);
arr.some(x => { return x === 1; });
arr.some(function (x) { return x === 1; });
!arr.some(x => x === 1);

// parenthesized receiver — should still be flagged
(arr).some(x => x === 1);
(arr).indexOf(1) !== -1;

// a static member access is stable and side-effect free, so it can be hoisted
// out of the callback into the includes() argument
arr.some(x => x === str.length);

// parenthesized callee — the whole `arr.some` is wrapped
(arr.some)(x => x === 1);
