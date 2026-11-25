/* should generate diagnostics */
{
	const index = foo.indexOf(bar);
	index < 0;
}
{
	const index = foo.indexOf(bar);
	index >= 0;
}
{
	const index = foo.indexOf(bar);
	index > -1;
}
{
	const index = foo.lastIndexOf(bar);
	if (index < 0) {}
}
{
	const index = foo.findIndex((element) => element > 10);
	if (index >= 0) {}
}
{
	const index = foo.findLastIndex((element) => element > 10);
	if (index > -1) {}
}
{
	const index = (foo.indexOf(bar));
	(index) < (0);
}
{
	const index = foo.indexOf(bar);
	index > -(1);
}
{
	const index = _.indexOf([1, 2, 1, 2], 2);
	index < 0;
}
{
	const index = foo.indexOf(bar);
	function inner() {
		if (index < 0) {}
	}
}
{
	const index1 = foo.indexOf("1"),
		index2 = foo.indexOf("2");
	index1 < 0;
	index2 >= 0;
}
{
	const index = (foo.indexOf)(bar);
	index < 0;
}
{
	const index = foo.indexOf("1");
	((
		/* comment 1 */
		((
			/* comment 2 */
			index
			/* comment 3 */
		))
		/* comment 4 */
		<
		/* comment 5 */
		((
			/* comment 6 */
			0
			/* comment 7 */
		))
		/* comment 8 */
	));
}
{
	const index = foo.indexOf("1");
	((
		/* comment 1 */
		((
			/* comment 2 */
			index
			/* comment 3 */
		))
		/* comment 4 */
		>
		((
			/* comment 5 */
			- /* comment 6 */ (( /* comment 7 */ 1 /* comment 8 */ ))
			/* comment 9 */
		))
	));
}
