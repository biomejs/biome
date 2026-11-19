/* should generate diagnostics */
function union(a: Set<string>, b: ReadonlySet<string>) {
	return new Set([...a, ...b]);
}

function intersection(a: Set<string>, b: Set<string>) {
	return [...a].filter((value: string) => b.has(value));
}

function difference(a: Set<string>, b: Set<string>) {
	return new Set([...a].filter((value) => !b.has(value)) as string[]);
}

function subset(a: Set<string>, b: Set<string> | ReadonlySet<string>) {
	return [...a].every((value) => b.has(value));
}

function asserted(a: unknown, b: unknown) {
	return [...(a as Set<string>)].some((value) => (b as Set<string>).has(value));
}

function sizes(a: Set<number>, b: Set<number>) {
	const disjoint = a.intersection(b).size === 0;
	const overlaps = a.intersection(b).size !== 0;
	return disjoint || overlaps;
}

function nonNull(a: Set<string>, b: Set<string>) {
	return [...a].some((value) => b.has(value))!;
}
