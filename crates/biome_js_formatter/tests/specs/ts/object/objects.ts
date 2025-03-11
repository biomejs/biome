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
