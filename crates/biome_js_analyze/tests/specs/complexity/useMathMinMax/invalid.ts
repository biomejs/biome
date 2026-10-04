/* should generate diagnostics */
function numericAssertion(a, b) {
    return (a as number) > b ? a : b;
}

function numericAssertionAgain(a, b) {
    return (a as number) > b ? a : b;
}

function nestedNumericAssertion(a, b) {
    return (a as unknown as number) > b ? a : b;
}

var foo = 10;
var inferredNumber = foo > bar ? bar : foo;

var leftNumber = 10;
var rightNumber = 20;
var bothNumbers = leftNumber > rightNumber ? rightNumber : leftNumber;

var annotatedFoo: number;
var annotatedBar: number;
var annotatedNumbers = annotatedFoo > annotatedBar ? annotatedBar : annotatedFoo;

function numericBranchAssertion(a, b) {
    return (a as number) > b ? (a as number) : b;
}

function numericAngleAssertion(a, b) {
    return (<number>a) > b ? (<number>a) : b;
}

function nonNullNumber(a, b) {
    return a! > b ? a! : b;
}
