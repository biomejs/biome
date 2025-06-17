let foo: {
	bar: string;
};

const fn = ({
	foo,
	bar,
}: {
	foo: boolean;
	bar: string;
}) => {
	console.log(foo, bar);
}

function fn2(foo: {
	bar: string;
}) {
	console.log(foo);
}

// both the object pattern and the object type should be expanded
function fn3({
	foo,
	bar,
	baz,
	qux,
}: {
	foo: string;
	baz: string;
	bar: string;
	qux: string;
}): void {
}

// the object type of `baz` should keep expanded
function fn4(
	bar: string,
	baz: {
		qux: string;
	}
): void {
}

// the object type of `baz` should be collapsed
function fn5(
	bar: string,
	baz: { qux: string;
	}
): void {
}

