/* should not generate diagnostics */
if (foo) {
	const bar = value as string;
} else {
	const bar = value as number;
}

if (foo) {
	const bar = <string>value;
} else {
	const bar = value as string;
}
