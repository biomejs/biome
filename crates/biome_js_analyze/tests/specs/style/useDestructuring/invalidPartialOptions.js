/* should generate diagnostics for object cases only */
/* array disabled, but object should still fire */
{
	var foo = array[0];
}
{
	var foo = object.foo;
}
{
	foo = array[0];
}
{
	foo = object.foo;
}
