var foo = 0 + 31;
a = a + 5;
a += 5;
var foo = 0 + 1 + -4 + 4;
var foo = 0 + 1 + 5;

console.log(0x1A + 0x02);
console.log(0o71); // Equivalent to the legacy-octal 071 case; kept parseable for action validation.

var colors = {}; colors.RED = 3; colors.YELLOW = 4; colors.BLUE = 4 + 5;
function getSecondsInMinute() {return 60;}
function getNegativeSecondsInMinute() {return -60;}

function getSecondsInDay() {
	return 24 * HOUR;
}
function getMillisecondsInDay() {
	return (getSecondsInDay() *
		(1000)
	);
}
function callLater(func) {
	setTimeout(func, 100);
}

var a = <div arrayProp={[1,2,3]}></div>;

function getMillisecondsSinceStart() {
	return getSecondsInDay() * 1000;
}

function schedule(work) {
	setTimeout(work, 100);
}

const existing = 1;
function usesFallback(value) {
	return value + 31;
}

const numericKeys = {};
numericKeys[7] = 31;

const bigintTotal = bigintValue + 123n;
const commented = value + /* keep */ 37;

const assertedNumber = (31 as const) + value;
const mixedAssertions = [31 as const, 31];
const nestedAssertion = ((31 as const) as number) + value;

type NumericLiteral = 31;
