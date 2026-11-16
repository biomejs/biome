/* should generate diagnostics */
{
	const index = list.indexOf(item);
	expected === index < 0;
	expected !== index >= 0;
	expected == index > -1;
	expected != index < 0;
}
{
	const index = list.indexOf(item);
	index < 0 < expected;
	index >= 0 === expected;
	expected === (index < 0);
	expected === ((index < 0));
	expected === index /* before */ < /* after */ 0;
	index < 0 && expected;
	index < 0 | mask;
	index < 0 in object;
	index < 0 instanceof Foo;
}
{
	const index = list.indexOf(item);
	foo()
	index < 0 < 1
}
{
	const index = list.indexOf(item);
	foo() // keep
	index < 0 < 1
}
{
	const index = list.indexOf(item);
	const previous = value
	index < 0 === expected
}
{
	const index = list.indexOf(item);
	foo();
	index < 0 < 1;
}
{
	const index = list.indexOf(item);
	if (ready)
	index < 0 < 1
}
