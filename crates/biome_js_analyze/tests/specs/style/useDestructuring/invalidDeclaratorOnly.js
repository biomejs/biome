/* should generate diagnostics */
/* assignment disabled, only declarations should fire */
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
