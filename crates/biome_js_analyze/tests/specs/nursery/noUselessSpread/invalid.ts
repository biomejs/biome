/* should generate diagnostics */
function consume(values: number[]) {
	for (const value of [...(values as number[])]) {
		values.pop();
	}
}
Object.fromEntries([...(values! satisfies Iterable<[string, number]>)]);
new [...Array.of(1)]!.constructor(2);
