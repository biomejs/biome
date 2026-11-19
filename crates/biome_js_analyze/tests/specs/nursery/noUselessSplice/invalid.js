/* should generate diagnostics */
array.splice(0, 1);
array.splice();
array.splice(index, 0);
array.splice(index, -1);
array.splice(index, +0);
array.splice(index, - -0);
array.splice(+0, +1);
array.splice(0, - -1);
array.splice(- -0, 1);
array.splice(0, 0, element);
array.splice(0, -1, element);
array.splice(0, 0, element, ...moreElements);
array.splice(0, 0, ...elements);
array.splice(array.length - 1, 1);
array.splice(array.length - +1, +1);
object.array.splice(object.array.length - 1, 1);
array.splice(array.length, 0, element);
array.splice(array.length, 0, element, ...moreElements);
array.splice(array.length, 0, ...elements);
object.array.splice(object.array.length, 0, element);
object["array"].splice(object.array.length, 0, element);
array.splice(0);
object.array.splice(0);
this.items.splice(0);
array.splice(0, array.length);
object.array.splice(0, object.array.length);

// Other kinds of objects
[1, 2, 3].splice(0, 1);
getArray().splice(0, 1);
object[foo?.bar].splice(index, 0);

// Parentheses, trailing commas, and line breaks
(array.splice)(0, 1);
(array).splice(0, 1);
(array.splice(0));
array.splice((0), (1));
array.splice(0, 1,);
array.splice(0, 0, element,);
array.splice(0, 0, (first, second));
array.splice(array.length, 0, (first, second));
array
	.splice(
		0,
		0,
		element,
	);

// The comma operator discards the value of every operand except the last one.
array.splice(0), f();
array.splice(0, array.length), f();
f(), array.splice(0);
f(), array.splice(0), g();
array.splice(index, 0), f();
array.splice(0, 1), f();

// The returned value is used, so there is no fix.
const noOpResult = array.splice(index, 0);
const shiftResult = array.splice(0, 1);
const unshiftResult = array.splice(0, 0, element);
const popResult = array.splice(array.length - 1, 1);
const pushResult = array.splice(array.length, 0, element);
consume((f(), array.splice(0, 1)));

// A fix would remove these comments.
array.splice(0, /* comment */ 0, element);
array.splice(/* comment */ 0, 1);
array.splice(index, /* comment */ 0);
array.splice(index, 0) /* comment */;
array.splice(index, 0); // comment
// comment
array.splice(index, 0);

// Removing these calls would also remove other function calls.
array.splice(getIndex(), 0);
getArray().splice(index, 0);
array.splice(index++, 0);

// Removing this call would leave an `if` statement without a body.
if (condition) array.splice(index, 0);

// `push()` would add the element at a different position if `createElement()` changes the array.
array.splice(array.length, 0, createElement());
array.splice(array.length, 0, ...createElements());
array.splice(0, 0, createElement());
