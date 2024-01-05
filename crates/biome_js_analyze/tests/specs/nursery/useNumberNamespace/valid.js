Number.parseInt("1");

Number.parseFloat("1.1");

Number.NaN;

Number.POSITIVE_INFINITY;

Number.NEGATIVE_INFINITY;

isFinite({});

isNaN({});

const { parseInt } = Number;
const foo = parseInt("10", 2);

function f(parseFloat) {
	parseFloat("1.1");
}
