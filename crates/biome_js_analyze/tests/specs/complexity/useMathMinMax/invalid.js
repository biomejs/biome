/* should generate diagnostics */
height > 50 ? 50 : height;
height >= 50 ? 50 : height;
height < 50 ? height : 50;
height <= 50 ? height : 50;

height > maxHeight ? maxHeight : height;
height < maxHeight ? height : maxHeight;

window.height > 50 ? 50 : window.height;
window.height < 50 ? window.height : 50;

height > 50 ? height : 50;
height >= 50 ? height : 50;
height < 50 ? 50 : height;
height <= 50 ? 50 : height;

height > maxHeight ? height : maxHeight;
height < maxHeight ? maxHeight : height;

function edgeOne() {
    return +foo > 10 ? 10 : +foo;
}

function edgeTwo() {
    return+foo > 10 ? 10 : +foo;
}

(0, foo) > 10 ? 10 : (0, foo);

foo.bar > 10 ? 10 : foo.bar;

async function asyncEdge() {
    return await foo.bar > 10 ? 10 : await foo.bar;
}

async function awaitCall() {
    await (+foo > 10 ? 10 : +foo);
}

function groupedTest() {
    return (foo.bar > 10) ? 10 : foo.bar;
}

function* yielded() {
    yield+foo > 10 ? 10 : +foo;
}

export default +foo > 10 ? 10 : +foo;

foo.length > bar.length ? bar.length : foo.length;

foo > /* keep */ bar ? bar : foo;
foo > bar ? /* keep */ bar : foo;

// comment before
alpha > beta ? alpha : beta
// comment after

lhs /* lhs in condition */ > rhs /* rhs in condition */
    ? lhs /* lhs in consequent */
    : /* rhs in alternate */ rhs;

// min before
minLeft > minRight ? minRight : minLeft
// min after

maxLeft /* max left in condition */ < maxRight /* max right in condition */
    ? /* max right in consequent */ maxRight
    : maxLeft /* max left in alternate */;

minLhs /* min lhs in condition */ > minRhs /* min rhs in condition */
    ? /* min rhs in consequent */ minRhs
    : minLhs /* min lhs in alternate */;

leftValue > rightValue
    ? leftValue /* selected left in consequent */
    : /* unselected right in alternate */ rightValue;

leftLimit /* left condition trailing */ <= rightLimit /* right condition trailing */
    ? leftLimit /* left selected in consequent */
    : /* right alternate leading */ rightLimit;

outerLeft > outerRight
    ? outerLeft
    : outerRight;
/* trailing block comment */

first > second ? first : second; // trailing line comment

commentedObject.left /* object left in condition */ > commentedObject.right /* object right in condition */
    ? commentedObject.left /* object left in consequent */
    : /* object right in alternate */ commentedObject.right;
