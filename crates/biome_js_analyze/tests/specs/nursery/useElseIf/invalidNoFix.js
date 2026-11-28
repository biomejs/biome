/* should generate diagnostics */
// An `else` after an `if` without its own `else` would belong to that inner `if`.
{
	if (x === 1) if (y) 0;
	if (x === 2) console.log("two");
}

{
	if (x === 1) while (y) if (y) 0;
	if (x === 2) console.log("two");
}

{
	if (x === 1) foo: if (y) 0;
	if (x === 2) console.log("two");
}

{
	if (x === 1) 1; else if (x === 3) if (y) 0;
	if (x === 2) console.log("two");
}

{
	if (x === 1) for (const a of b) if (y) 0;
	if (x === 2) console.log("two");
}
