/* should generate diagnostics */

// Presence checks (positive includes)
str.indexOf("foo") !== -1;
arr.indexOf(item) != -1;
str.indexOf("foo") >= 0;
str.indexOf("foo") > -1;

// Absence checks (negated includes)
str.indexOf("foo") === -1;
arr.indexOf(item) == -1;
str.indexOf("foo") < 0;

// Reversed operands
-1 !== str.indexOf("foo");
-1 != arr.indexOf(item);
0 <= str.indexOf("foo");
-1 < str.indexOf("foo");
-1 === str.indexOf("foo");
-1 == arr.indexOf(item);
0 > str.indexOf("foo");

// With parentheses
(str.indexOf("foo")) !== -1;
str.indexOf("foo") !== (-1);

// Computed member expressions
foo.bar.indexOf(baz) !== -1;
