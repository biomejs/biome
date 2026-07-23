/* should generate diagnostics */
/* declarator: array only, assignment: object only */
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
