/* should generate diagnostics */
{
	const index = list.indexOf(item);
	index < 0 as boolean;
}
{
	const index = list.indexOf(item);
	index < 0 satisfies boolean;
}
{
	const index = list.indexOf(item);
	foo()
	index < 0 as boolean
}
{
	const index: number = list.indexOf(item);
	index < 0;
}
{
	const index = list.indexOf(item);
	type index = number;
	if (index < 0) {}
}
