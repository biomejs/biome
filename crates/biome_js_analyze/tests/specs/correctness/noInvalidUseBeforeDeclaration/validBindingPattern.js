function f({ a = 0, b = a }) {}
function f([a = 0, b = a]) {}
function f({ x: [{ a = 0 }, b = a] }) {}
{
	const { a = 0, b = a } = {};
}
{
	const [a = 0, b = a] = {};
}
{
	const {
		x: [{ a = 0 }, b = a],
	} = {};
}
