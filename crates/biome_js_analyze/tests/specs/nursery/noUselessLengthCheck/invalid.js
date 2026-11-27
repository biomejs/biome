/* should generate diagnostics */

// Before `every()` and `some()`
array.length === 0 || array.every(Boolean);
array.length > 0 && array.some(Boolean);
array.length !== 0 && array.some(Boolean);
if ((( array.length > 0 )) && array.some(Boolean));
const isAllTruthyOrEmpty = array.length === 0 || array.every(Boolean);
((
	((
		(( array )).length
	)) === (( 0 ))
	||
	((
		(( array )).every(Boolean)
	))
));
array.length > 0 && (array.some)(Boolean);
object.items.length > 0 && object.items.some(Boolean);
object["items"].length > 0 && object.items.some(Boolean);
this.length === 0 || this.every(Boolean);
array.length > 0 && array.some(createPredicate());

// After `every()` and `some()`
array.every(Boolean) || array.length === 0;
array.some(Boolean) && array.length !== 0;
array.some(Boolean) && array.length > 0;
array.some((value) => array.pop()) && array.length > 0;
((
	((
		(( array )).every(Boolean)
	))
	||
	((
		(( array )).length
	)) === (( 0 ))
));

// Inside a longer chain of the same operator
if (
	array.length !== 0 &&
	array.some(Boolean) &&
	foo
) {
	// ...
}
(array.length === 0 || array.every(Boolean)) || foo;
foo || (array.length === 0 || array.every(Boolean));
(array.length > 0 && array.some(Boolean)) && foo;
foo && (array.length > 0 && array.some(Boolean));
foo && array.length > 0 && array.some(Boolean);
foo || array.length === 0 || array.every(Boolean);
(foo || array.length === 0) || array.every(Boolean);
array.length === 0 || (array.every(Boolean) || foo);
(foo && array.length > 0) && array.some(Boolean);
array.length > 0 && (array.some(Boolean) && foo);
array.every(Boolean) || array.length === 0 || array.every(Boolean);
array.some(Boolean) && array.length > 0 && array.some(Boolean);
array.length === 0 || array.every(Boolean) || array.length === 0;
array1.every(Boolean)
|| (( array1.length === 0 || array2.length === 0 )) // Both are unnecessary
|| array2.every(Boolean);
function isUselessLengthCheckNode({node, operator, siblings}) {
	return (
		(
			operator === '||' &&
			zeroLengthChecks.has(node) &&
			siblings.length > 0 &&
			siblings.some(condition =>
				arrayEveryCalls.has(condition) &&
				isSameReference(node.left.object, condition.callee.object)
			)
		) ||
		(
			operator === '&&' &&
			nonZeroLengthChecks.has(node) &&
			siblings.length > 0 &&
			siblings.some(condition =>
				arraySomeCalls.has(condition) &&
				isSameReference(node.left.object, condition.callee.object)
			)
		)
	);
}

// Comments
/* keep */ array.length !== 0 && array.some(Boolean);
array.length !== 0 && array.some(/* keep */ Boolean);
array.some(Boolean) && array.length !== 0; // keep
array.length !== 0 && /* removed */ array.some(Boolean);
array.some(Boolean) /* removed */ && array.length !== 0;

// Statements without semicolons
foo()
array.length > 0 && array.some(Boolean)
foo()
array.length > 0 && (array).some(Boolean)
foo();
array.length > 0 && (array).some(Boolean)

// Before `for...of`
if (array.length > 0) {
	for (const element of array) {
		use(element);
	}
}
if (array.length !== 0) {
	for (const element of array) {
		// Do work.
	}
}
function iterate(array) {
	if (array.length > 0) {
		for (const element of array) {
			use(element);
		}
	}
}
if (array.length > 0) for (const element of array) { use(element); }
if (array.length > 0) { for (const element of array) use(element); }
if (((array.length > 0))) { for (const element of array) { use(element); } }
if (object.array.length > 0) { for (const element of object.array) { use(element); } }
if (array.length > 0) { for (const [key, value] of array) { use(key, value); } }
if (array.length > 0) { for (element of array) { use(element); } }
function iterateVar(array) { if (array.length > 0) { for (var array of array) {} } }
if (arrays.length > 0) {
	for (const array of arrays) {
		if (array.length > 0) {
			for (const element of array) {
				use(element);
			}
		}
	}
}
// leading comment
if (array.length > 0) {
	for (const element of array) {
		use(element);
	}
} // trailing comment
if (array.length > 0) { /* removed */ for (const element of array) { use(element); } }
if (array.length /* removed */ > 0) { for (const element of array) { use(element); } }
