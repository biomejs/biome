function f({ b = a, a = 0 }) {}
function f([b = a, a = 0]) {}
function f({ x: [b = a, { a = 0 }] }) {}
function f({ a = a }) {}
{
	const { b = a, a = 0 } = {};
}
{
	const [b = a, a = 0] = {};
}
{
	const {
		x: [b = a, { a = 0 }],
	} = {};
}