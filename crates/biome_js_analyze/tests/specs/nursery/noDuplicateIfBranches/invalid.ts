/* should generate diagnostics */
if (foo) {
	const bar = value as string;
	baz(bar);
} else {
	const bar = value as string;
	baz(bar);
}

if (foo) {
	const bar = value satisfies string;
	baz(bar!);
} else {
	const bar = value satisfies string;
	baz(bar!);
}
