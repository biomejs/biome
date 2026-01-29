// =====================
// Array expressions
// =====================

// Basic arrays
const arr1 = [a, b, c];
const arr2 = [x];
const arr3 = [];
const arr4 = [...a, ...b];

// Nested arrays with single element stay on one line
const arr5 = [[a], [b]];

// Nested arrays with 2+ elements break
const arr6 = [[a, b], [c, d]];

// Nested objects with single property stay on one line
const arr7 = [{a: 1}, {b: 2}];

// Nested objects with 2+ properties break
const arr8 = [{a: 1, b: 2}, {c: 3, d: 4}];

// Boundary: 78 chars + 2 spaces = 80 (fits)
const arr9 = [ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo, b];

// Boundary: 80 chars + 2 spaces = 82 (breaks)
const arr10 = [oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo, b];

// =====================
// Array assignment patterns
// =====================

// Basic array assignment pattern
[a, b, c] = arr;

// Single element assignment
[a] = arr;

// Assignment with rest element
[a, ...b] = arr;

// Assignment with default values
[a = 1, b = 2] = arr;

// Assignment with holes
[, a] = arr;
[, , a] = arr;

// Nested array assignment
[[a, b], [c, d]] = arr;

// Mixed nested assignment
[{a}, [b]] = arr;

// =====================
// Array binding patterns
// =====================

// Basic array binding in const
const [x, y, z] = arr;

// Array binding with rest
const [a, ...b] = arr;

// Array binding with default values
const [a = 1, b = 2] = arr;

// Array binding with holes
const [, b] = arr;
const [, , c] = arr;

// Nested array binding
const [[a, b], [c]] = arr;

// Mixed nested binding
const [{a}, [b]] = arr;

// Array binding in let
let [a, b] = arr;

// Array binding in var
var [a, b] = arr;

// Array binding in function parameter (covered in parameters)
function foo([a, b]) {}

// Array binding in arrow function parameter
const fn = ([a, b]) => {};

// Array binding in for-of loop
for (const [a, b] of arr) {}

// Array binding in catch clause (covered in catch)
try {} catch ([e]) {}
