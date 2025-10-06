var foo = 0 + 31;
a = a + 5;
a += 5;
var foo = 0 + 1 + -4 + 4;
var foo = 0 + 1 + 5;

console.log(0x1A + 0x02);
console.log(071);

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


