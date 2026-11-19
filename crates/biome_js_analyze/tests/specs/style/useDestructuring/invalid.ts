/* should generate diagnostics */

// No type annotation — should still suggest destructuring
{
	const foo = object.foo;
}
{
	let foo = array[0];
}
{
	var foo = object['foo'];
}
