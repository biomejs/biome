/* should not generate diagnostics */
{
	const index = list.indexOf(item) as number;
	index < 0;
}
{
	const index = list.indexOf(item);
	(index as number) < 0;
}
